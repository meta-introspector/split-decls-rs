// Generated macro for Syminfo32 (struct)
macro_rules! Depcrate_elfSyminfo32 {
() => {
// Module: crate::elf
// Provides: {"Syminfo32"}
// Dependencies: {}
# [doc = " Additional information about a `Sym32`."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Syminfo32 < E : Endian > { # [doc = " Direct bindings, symbol bound to."] pub si_boundto : U16 < E > , # [doc = " Per symbol flags."] pub si_flags : U16 < E > , }
};
}
