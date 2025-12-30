// Generated macro for impl_121 (impl)
macro_rules! Depcrate_fetch_typesimpl_121 {
() => {
// Module: crate::fetch::types
// Provides: {"impl_121"}
// Dependencies: {}
impl Shallow { # [doc = " Produce a variant that causes the repository to loose its shallow boundary, effectively by extending it"] # [doc = " beyond all limits."] pub fn undo () -> Self { Shallow :: DepthAtRemote ((i32 :: MAX as u32) . try_into () . expect ("valid at compile time")) } }
};
}
