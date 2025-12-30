// Generated macro for FeeDetails (struct)
macro_rules! DepcrateFeeDetails {
() => {
// Module: crate
// Provides: {"FeeDetails"}
// Dependencies: {}
# [cfg_attr (feature = "serde" , derive (serde_derive :: Deserialize , serde_derive :: Serialize))] # [derive (Debug , Default , Clone , Copy , Eq , PartialEq)] pub struct FeeDetails { transaction_fee : u64 , prioritization_fee : u64 , }
};
}
