// Generated macro for impl_305 (impl)
macro_rules! Depcrate_wakerimpl_305 {
() => {
// Module: crate::waker
// Provides: {"impl_305"}
// Dependencies: {}
impl Drop for Waker { # [inline] fn drop (& mut self) { debug_assert_eq ! (self . selectors . len () , 0) ; debug_assert_eq ! (self . observers . len () , 0) ; } }
};
}
