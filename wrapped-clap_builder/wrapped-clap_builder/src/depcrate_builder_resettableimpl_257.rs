// Generated macro for impl_257 (impl)
macro_rules! Depcrate_builder_resettableimpl_257 {
() => {
// Module: crate::builder::resettable
// Provides: {"impl_257"}
// Dependencies: {}
impl IntoResettable < char > for Option < char > { fn into_resettable (self) -> Resettable < char > { match self { Some (s) => Resettable :: Value (s) , None => Resettable :: Reset , } } }
};
}
