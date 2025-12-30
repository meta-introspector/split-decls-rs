// Generated macro for Verdaux (struct)
macro_rules! Depcrate_elfVerdaux {
() => {
// Module: crate::elf
// Provides: {"Verdaux"}
// Dependencies: {}
# [doc = " Auxiliary version information."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Verdaux < E : Endian > { # [doc = " Version or dependency names"] pub vda_name : U32 < E > , # [doc = " Offset in bytes to next verdaux"] pub vda_next : U32 < E > , }
};
}
