// Generated macro for FeeRateGovernor (struct)
macro_rules! DepcrateFeeRateGovernor {
() => {
// Module: crate
// Provides: {"FeeRateGovernor"}
// Dependencies: {}
# [cfg_attr (feature = "frozen-abi" , derive (solana_frozen_abi_macro :: AbiExample))] # [cfg_attr (feature = "serde" , derive (serde_derive :: Serialize , serde_derive :: Deserialize))] # [derive (PartialEq , Eq , Clone , Debug)] # [cfg_attr (feature = "serde" , serde (rename_all = "camelCase"))] pub struct FeeRateGovernor { # [cfg_attr (feature = "serde" , serde (skip))] pub lamports_per_signature : u64 , pub target_lamports_per_signature : u64 , pub target_signatures_per_slot : u64 , pub min_lamports_per_signature : u64 , pub max_lamports_per_signature : u64 , pub burn_percent : u8 , }
};
}
