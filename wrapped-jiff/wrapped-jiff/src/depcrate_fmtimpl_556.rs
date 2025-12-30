// Generated macro for impl_556 (impl)
macro_rules! Depcrate_fmtimpl_556 {
() => {
// Module: crate::fmt
// Provides: {"impl_556"}
// Dependencies: {}
# [cfg (any (test , feature = "alloc"))] impl Write for alloc :: vec :: Vec < u8 > { # [inline] fn write_str (& mut self , string : & str) -> Result < () , Error > { self . extend_from_slice (string . as_bytes ()) ; Ok (()) } }
};
}
