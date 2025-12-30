// Generated macro for impl_260 (impl)
macro_rules! Depcrate_builder_resettableimpl_260 {
() => {
// Module: crate::builder::resettable
// Provides: {"impl_260"}
// Dependencies: {}
impl IntoResettable < ValueHint > for Option < ValueHint > { fn into_resettable (self) -> Resettable < ValueHint > { match self { Some (s) => Resettable :: Value (s) , None => Resettable :: Reset , } } }
};
}
