// Generated macro for impl_94 (impl)
macro_rules! Depcrate_data_providerimpl_94 {
() => {
// Module: crate::data_provider
// Provides: {"impl_94"}
// Dependencies: {}
impl CGDataProviderRef { # [doc = " Creates a copy of the data from the underlying `CFDataProviderRef`."] pub fn copy_data (& self) -> CFData { unsafe { CFData :: wrap_under_create_rule (CGDataProviderCopyData (self . as_ptr ())) } } }
};
}
