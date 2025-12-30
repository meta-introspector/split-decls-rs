// Generated macro for Syminfo64 (struct)
macro_rules! Depcrate_elfSyminfo64 {
() => {
// Module: crate::elf
// Provides: {"Syminfo64"}
// Dependencies: {}
# [doc = " Additional information about a `Sym64`."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Syminfo64 < E : Endian > { # [doc = " Direct bindings, symbol bound to."] pub si_boundto : U16 < E > , # [doc = " Per symbol flags."] pub si_flags : U16 < E > , }
};
}
