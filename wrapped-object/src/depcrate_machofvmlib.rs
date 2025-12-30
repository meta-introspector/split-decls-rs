// Generated macro for Fvmlib (struct)
macro_rules! Depcrate_machoFvmlib {
() => {
// Module: crate::macho
// Provides: {"Fvmlib"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Fvmlib < E : Endian > { # [doc = " library's target pathname"] pub name : LcStr < E > , # [doc = " library's minor version number"] pub minor_version : U32 < E > , # [doc = " library's header address"] pub header_addr : U32 < E > , }
};
}
