// Generated macro for impl_258 (impl)
macro_rules! Depcrate_builder_resettableimpl_258 {
() => {
// Module: crate::builder::resettable
// Provides: {"impl_258"}
// Dependencies: {}
impl IntoResettable < usize > for Option < usize > { fn into_resettable (self) -> Resettable < usize > { match self { Some (s) => Resettable :: Value (s) , None => Resettable :: Reset , } } }
};
}
