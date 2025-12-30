// Generated macro for impl_19 (impl)
macro_rules! Depcrate_arrayimpl_19 {
() => {
// Module: crate::array
// Provides: {"impl_19"}
// Dependencies: {}
impl < T > Drop for CFArray < T > { fn drop (& mut self) { unsafe { CFRelease (self . as_CFTypeRef ()) } } }
};
}
