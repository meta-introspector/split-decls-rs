// Generated macro for Encoding (struct)
macro_rules! Depcrate_commonEncoding {
() => {
// Module: crate::common
// Provides: {"Encoding"}
// Dependencies: {}
# [doc = " Encoding parameters that are commonly used for multiple DWARF sections."] # [doc = ""] # [doc = " This is intended to be small enough to pass by value."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [repr (C)] pub struct Encoding { # [doc = " The size of an address."] pub address_size : u8 , # [doc = " Whether the DWARF format is 32- or 64-bit."] pub format : Format , # [doc = " The DWARF version of the header."] pub version : u16 , }
};
}
