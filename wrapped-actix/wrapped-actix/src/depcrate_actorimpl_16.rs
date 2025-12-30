// Generated macro for impl_16 (impl)
macro_rules! Depcrate_actorimpl_16 {
() => {
// Module: crate::actor
// Provides: {"impl_16"}
// Dependencies: {}
impl SpawnHandle { # [doc = " Gets the next handle."] pub fn next (self) -> SpawnHandle { SpawnHandle (self . 0 + 1) } # [doc (hidden)] pub fn into_usize (self) -> usize { self . 0 } }
};
}
