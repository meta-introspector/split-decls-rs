// Generated macro for impl_54 (impl)
macro_rules! Depcrate_store_impls_dynamic_iterimpl_54 {
() => {
// Module: crate::store_impls::dynamic::iter
// Provides: {"impl_54"}
// Dependencies: {}
impl dynamic :: Store { # [doc = " Like [`Handle::iter()`][super::Handle::iter()], but accessible directly on the store."] pub fn iter (& self) -> Result < AllObjects , dynamic :: load_index :: Error > { AllObjects :: new (self) } }
};
}
