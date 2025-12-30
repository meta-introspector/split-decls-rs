// Generated macro for impl_130 (impl)
macro_rules! Depcrate_waiterimpl_130 {
() => {
// Module: crate::waiter
// Provides: {"impl_130"}
// Dependencies: {}
impl Drop for Waiter { fn drop (& mut self) { unsafe { WaitForSingleObject (self . 0 , 0xFFFFFFFF) ; CloseHandle (self . 0) ; } } }
};
}
