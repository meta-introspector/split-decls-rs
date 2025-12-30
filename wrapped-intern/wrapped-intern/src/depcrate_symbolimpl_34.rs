// Generated macro for impl_34 (impl)
macro_rules! Depcrate_symbolimpl_34 {
() => {
// Module: crate::symbol
// Provides: {"impl_34"}
// Dependencies: {}
impl Drop for Symbol { # [inline] fn drop (& mut self) { let Some (arc) = (unsafe { self . repr . try_as_arc_owned () }) else { return ; } ; if Arc :: count (& arc) == 2 { Self :: drop_slow (& arc) ; } ManuallyDrop :: into_inner (arc) ; } }
};
}
