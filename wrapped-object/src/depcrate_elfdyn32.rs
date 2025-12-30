// Generated macro for Dyn32 (struct)
macro_rules! Depcrate_elfDyn32 {
() => {
// Module: crate::elf
// Provides: {"Dyn32"}
// Dependencies: {}
# [doc = " Dynamic section entry."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Dyn32 < E : Endian > { # [doc = " Dynamic entry type."] pub d_tag : U32 < E > , # [doc = " Value (integer or address)."] pub d_val : U32 < E > , }
};
}
