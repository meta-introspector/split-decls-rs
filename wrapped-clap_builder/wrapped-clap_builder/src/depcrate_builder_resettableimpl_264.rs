// Generated macro for impl_264 (impl)
macro_rules! Depcrate_builder_resettableimpl_264 {
() => {
// Module: crate::builder::resettable
// Provides: {"impl_264"}
// Dependencies: {}
impl IntoResettable < Str > for Option < & 'static str > { fn into_resettable (self) -> Resettable < Str > { match self { Some (s) => Resettable :: Value (s . into ()) , None => Resettable :: Reset , } } }
};
}
