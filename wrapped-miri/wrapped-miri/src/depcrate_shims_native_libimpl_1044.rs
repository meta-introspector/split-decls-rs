// Generated macro for impl_1044 (impl)
macro_rules! Depcrate_shims_native_libimpl_1044 {
() => {
// Module: crate::shims::native_lib
// Provides: {"impl_1044"}
// Dependencies: {}
impl AccessEvent { fn get_range (& self) -> AccessRange { match self { AccessEvent :: Read (access_range) => access_range . clone () , AccessEvent :: Write (access_range , _) => access_range . clone () , } } }
};
}
