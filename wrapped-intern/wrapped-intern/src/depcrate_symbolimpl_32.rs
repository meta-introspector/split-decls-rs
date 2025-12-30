// Generated macro for impl_32 (impl)
macro_rules! Depcrate_symbolimpl_32 {
() => {
// Module: crate::symbol
// Provides: {"impl_32"}
// Dependencies: {}
impl Drop for Symbol { # [inline] fn drop (& mut self) { let Some (arc) = (unsafe { self . repr . try_as_arc_owned () }) else { return ; } ; if Arc :: count (& arc) == 2 { Self :: drop_slow (self) ; } ManuallyDrop :: into_inner (arc) ; } }
};
}
