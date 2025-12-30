// Generated macro for AddressSize (enum)
macro_rules! Depcrate_commonAddressSize {
() => {
// Module: crate::common
// Provides: {"AddressSize"}
// Dependencies: {}
# [doc = " The size of an address value for an architecture."] # [doc = ""] # [doc = " This may differ from the address size supported by the file format (such as for COFF)."] # [allow (missing_docs)] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [non_exhaustive] # [repr (u8)] pub enum AddressSize { U8 = 1 , U16 = 2 , U32 = 4 , U64 = 8 , }
};
}
