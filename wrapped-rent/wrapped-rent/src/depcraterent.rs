// Generated macro for Rent (struct)
macro_rules! DepcrateRent {
() => {
// Module: crate
// Provides: {"Rent"}
// Dependencies: {}
# [doc = " Configuration of network rent."] # [repr (C)] # [cfg_attr (feature = "frozen-abi" , derive (solana_frozen_abi_macro :: AbiExample))] # [cfg_attr (feature = "serde" , derive (serde_derive :: Deserialize , serde_derive :: Serialize))] # [derive (PartialEq , CloneZeroed , Debug)] pub struct Rent { # [doc = " Rental rate in lamports/byte-year."] # [deprecated (since = "3.1.0" , note = "Field will be renamed to `lamports_per_byte` in v4, use `Rent::new_with_lamports_per_byte` to create, and `Rent::minimum_balance` or `Rent::is_exempt`")] pub lamports_per_byte_year : u64 , # [doc = " Amount of time (in years) a balance must include rent for the account to"] # [doc = " be rent exempt."] # [deprecated (since = "3.1.0" , note = "Exemption threshold will be set to 1f64 with SIMD-0194 and should no longer be used")] pub exemption_threshold : f64 , # [doc = " The percentage of collected rent that is burned."] # [doc = ""] # [doc = " Valid values are in the range [0, 100]. The remaining percentage is"] # [doc = " distributed to validators."] # [deprecated (since = "3.1.0" , note = "The concept of rent no longer exists, only rent-exemption")] pub burn_percent : u8 , }
};
}
