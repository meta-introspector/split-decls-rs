// Generated macro for impl_121 (impl)
macro_rules! Depcrate_bytesimpl_121 {
() => {
// Module: crate::bytes
// Provides: {"impl_121"}
// Dependencies: {}
impl Drop for Bytes { # [inline] fn drop (& mut self) { unsafe { (self . vtable . drop) (& mut self . data , self . ptr , self . len) } } }
};
}
