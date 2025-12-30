// Generated macro for GenesisConfig (struct)
macro_rules! DepcrateGenesisConfig {
() => {
// Module: crate
// Provides: {"GenesisConfig"}
// Dependencies: {}
# [cfg_attr (feature = "frozen-abi" , derive (AbiExample) , frozen_abi (digest = "3tUUJkZiUUGfuNCXbDuDR6KCQYPsh3m3cPw5vVUSt113"))] # [cfg_attr (feature = "serde" , derive (serde_derive :: Deserialize , serde_derive :: Serialize))] # [derive (Clone , Debug , PartialEq)] pub struct GenesisConfig { # [doc = " when the network (bootstrap validator) was started relative to the UNIX Epoch"] pub creation_time : UnixTimestamp , # [doc = " initial accounts"] pub accounts : BTreeMap < Pubkey , Account > , # [doc = " built-in programs"] pub native_instruction_processors : Vec < (String , Pubkey) > , # [doc = " accounts for network rewards, these do not count towards capitalization"] pub rewards_pools : BTreeMap < Pubkey , Account > , pub ticks_per_slot : u64 , pub unused : u64 , # [doc = " network speed configuration"] pub poh_config : PohConfig , # [doc = " this field exists only to ensure that the binary layout of GenesisConfig remains compatible"] # [doc = " with the Solana v0.23 release line"] pub __backwards_compat_with_v0_23 : u64 , # [doc = " transaction fee config"] pub fee_rate_governor : FeeRateGovernor , # [doc = " rent config"] pub rent : Rent , # [doc = " inflation config"] pub inflation : Inflation , # [doc = " how slots map to epochs"] pub epoch_schedule : EpochSchedule , # [doc = " network runlevel"] pub cluster_type : ClusterType , }
};
}
