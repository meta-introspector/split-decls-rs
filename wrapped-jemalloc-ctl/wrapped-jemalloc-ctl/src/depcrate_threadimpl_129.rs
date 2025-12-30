// Generated macro for impl_129 (impl)
macro_rules! Depcrate_threadimpl_129 {
() => {
// Module: crate::thread
// Provides: {"impl_129"}
// Dependencies: {}
impl < T > ThreadLocal < T > where T : Copy , { # [doc = " Returns the current value at the pointer."] # [inline] pub fn get (self) -> T { unsafe { * self . 0 } } }
};
}
