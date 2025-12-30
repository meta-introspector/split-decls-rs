// Generated macro for FeeCalculator (struct)
macro_rules! DepcrateFeeCalculator {
() => {
// Module: crate
// Provides: {"FeeCalculator"}
// Dependencies: {}
# [repr (C)] # [cfg_attr (feature = "frozen-abi" , derive (solana_frozen_abi_macro :: AbiExample))] # [cfg_attr (feature = "serde" , derive (serde_derive :: Serialize , serde_derive :: Deserialize))] # [derive (Default , PartialEq , Eq , Clone , Copy , Debug)] # [cfg_attr (feature = "serde" , serde (rename_all = "camelCase"))] pub struct FeeCalculator { # [doc = " The current cost of a signature."] # [doc = ""] # [doc = " This amount may increase/decrease over time based on cluster processing"] # [doc = " load."] pub lamports_per_signature : u64 , }
};
}
