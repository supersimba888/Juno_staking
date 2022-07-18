use cosmwasm_std::{ Uint128};

use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


pub const CONFIG: Item<State> = Item::new("config_state");
pub const TOKENINFO : Map<&str,TokenInfo> = Map::new("config_nfts");
pub const OWNEDTOKEN : Map<&str, Vec<String>> = Map::new("config_owned");
pub const CLAIMFLAG : Map<&str,bool> = Map::new("claim_config");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner:String,
    pub denom:String,
    pub staking_period : u64,
    pub reward_wallet : String,
    pub total_staked : Uint128,
    pub nft_address : String,
    pub token_address : String,
    pub can_stake : bool,
    pub last_distribute:u64,
    pub distribute_period:u64,
    pub monthly_reward:Uint128,
    pub claim_reward:Uint128
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TokenInfo {
    pub owner : String,
    pub token_id: String,
    pub status : String,
    pub unstake_time:u64,
    pub stake_time:u64,
    pub reward:Uint128
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Image {   
    pub image: Option<String>,
}
