// Generated macro for impl_278 (impl)
macro_rules! Depcrate_setimpl_278 {
() => {
// Module: crate::set
// Provides: {"impl_278"}
// Dependencies: {}
impl < T > Drop for CFSet < T > { fn drop (& mut self) { unsafe { CFRelease (self . as_CFTypeRef ()) } } }
};
}
