// Generated macro for macro_116 (macro)
macro_rules! Depcrate_per_nsmacro_116 {
() => {
// Module: crate::per_ns
// Provides: {"macro_116"}
// Dependencies: {}
bitflags ! { # [doc = " Describes only the presence/absence of each namespace, without its value."] # [derive (Debug , PartialEq , Eq)] pub (crate) struct NsAvailability : u32 { const TYPES = 1 << 0 ; const VALUES = 1 << 1 ; const MACROS = 1 << 2 ; } }
};
}
