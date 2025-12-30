// Generated macro for AddressLookupTableAccount (struct)
macro_rules! DepcrateAddressLookupTableAccount {
() => {
// Module: crate
// Provides: {"AddressLookupTableAccount"}
// Dependencies: {}
# [doc = " The definition of address lookup table accounts."] # [doc = ""] # [doc = " As used by the `crate::v0` message format."] # [derive (Debug , PartialEq , Eq , Clone)] pub struct AddressLookupTableAccount { pub key : Address , pub addresses : Vec < Address > , }
};
}
