// Generated macro for PohConfig (struct)
macro_rules! DepcratePohConfig {
() => {
// Module: crate
// Provides: {"PohConfig"}
// Dependencies: {}
# [cfg_attr (feature = "frozen-abi" , derive (solana_frozen_abi_macro :: AbiExample))] # [cfg_attr (feature = "serde" , derive (serde_derive :: Deserialize , serde_derive :: Serialize))] # [derive (Clone , Debug , Eq , PartialEq)] pub struct PohConfig { # [doc = " The target tick rate of the cluster."] pub target_tick_duration : Duration , # [doc = " The target total tick count to be produced; used for testing only"] pub target_tick_count : Option < u64 > , # [doc = " How many hashes to roll before emitting the next tick entry."] # [doc = " None enables \"Low power mode\", which makes the validator sleep"] # [doc = " for `target_tick_duration` instead of hashing"] pub hashes_per_tick : Option < u64 > , }
};
}
