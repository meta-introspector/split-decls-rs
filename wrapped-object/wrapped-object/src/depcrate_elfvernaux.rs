// Generated macro for Vernaux (struct)
macro_rules! Depcrate_elfVernaux {
() => {
// Module: crate::elf
// Provides: {"Vernaux"}
// Dependencies: {}
# [doc = " Auxiliary needed version information."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Vernaux < E : Endian > { # [doc = " Hash value of dependency name"] pub vna_hash : U32 < E > , # [doc = " Dependency specific information"] pub vna_flags : U16 < E > , # [doc = " Version Index"] pub vna_other : U16 < E > , # [doc = " Dependency name string offset"] pub vna_name : U32 < E > , # [doc = " Offset in bytes to next vernaux entry"] pub vna_next : U32 < E > , }
};
}
