// Generated macro for impl_261 (impl)
macro_rules! Depcrate_builder_resettableimpl_261 {
() => {
// Module: crate::builder::resettable
// Provides: {"impl_261"}
// Dependencies: {}
impl IntoResettable < ValueParser > for Option < ValueParser > { fn into_resettable (self) -> Resettable < ValueParser > { match self { Some (s) => Resettable :: Value (s) , None => Resettable :: Reset , } } }
};
}
