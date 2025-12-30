// Generated macro for AddressLookupError (enum)
macro_rules! Depcrate_errorAddressLookupError {
() => {
// Module: crate::error
// Provides: {"AddressLookupError"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq , Clone)] pub enum AddressLookupError { # [doc = " Attempted to lookup addresses from a table that does not exist"] LookupTableAccountNotFound , # [doc = " Attempted to lookup addresses from an account owned by the wrong program"] InvalidAccountOwner , # [doc = " Attempted to lookup addresses from an invalid account"] InvalidAccountData , # [doc = " Address lookup contains an invalid index"] InvalidLookupIndex , }
};
}
