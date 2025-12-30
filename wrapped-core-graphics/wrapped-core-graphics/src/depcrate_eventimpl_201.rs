// Generated macro for impl_201 (impl)
macro_rules! Depcrate_eventimpl_201 {
() => {
// Module: crate::event
// Provides: {"impl_201"}
// Dependencies: {}
impl Drop for CGEventTap < '_ > { fn drop (& mut self) { unsafe { CFMachPortInvalidate (self . mach_port . as_CFTypeRef () as * mut _) } ; } }
};
}
