// Generated macro for Dylib (struct)
macro_rules! Depcrate_machoDylib {
() => {
// Module: crate::macho
// Provides: {"Dylib"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Dylib < E : Endian > { # [doc = " library's path name"] pub name : LcStr < E > , # [doc = " library's build time stamp"] pub timestamp : U32 < E > , # [doc = " library's current version number"] pub current_version : U32 < E > , # [doc = " library's compatibility vers number"] pub compatibility_version : U32 < E > , }
};
}
