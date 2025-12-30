// Generated macro for AddressError (enum)
macro_rules! Depcrate_errorAddressError {
() => {
// Module: crate::error
// Provides: {"AddressError"}
// Dependencies: {}
# [cfg_attr (feature = "serde" , derive (serde_derive :: Serialize))] # [derive (Debug , Clone , PartialEq , Eq)] pub enum AddressError { # [doc = " Length of the seed is too long for address generation"] MaxSeedLengthExceeded , InvalidSeeds , IllegalOwner , }
};
}
