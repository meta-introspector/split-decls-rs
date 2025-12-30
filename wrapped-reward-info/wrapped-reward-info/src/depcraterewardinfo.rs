// Generated macro for RewardInfo (struct)
macro_rules! DepcrateRewardInfo {
() => {
// Module: crate
// Provides: {"RewardInfo"}
// Dependencies: {}
# [cfg_attr (feature = "frozen-abi" , derive (AbiExample))] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] # [derive (Debug , PartialEq , Eq , Clone , Copy)] pub struct RewardInfo { pub reward_type : RewardType , # [doc = " Reward amount"] pub lamports : i64 , # [doc = " Account balance in lamports after `lamports` was applied"] pub post_balance : u64 , # [doc = " Vote account commission when the reward was credited, only present for voting and staking rewards"] pub commission : Option < u8 > , }
};
}
