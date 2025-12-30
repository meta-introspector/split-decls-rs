// Generated macro for Dyn64 (struct)
macro_rules! Depcrate_elfDyn64 {
() => {
// Module: crate::elf
// Provides: {"Dyn64"}
// Dependencies: {}
# [doc = " Dynamic section entry."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Dyn64 < E : Endian > { # [doc = " Dynamic entry type."] pub d_tag : U64 < E > , # [doc = " Value (integer or address)."] pub d_val : U64 < E > , }
};
}
