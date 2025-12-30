// Generated macro for impl_262 (impl)
macro_rules! Depcrate_builder_resettableimpl_262 {
() => {
// Module: crate::builder::resettable
// Provides: {"impl_262"}
// Dependencies: {}
impl IntoResettable < StyledStr > for Option < & 'static str > { fn into_resettable (self) -> Resettable < StyledStr > { match self { Some (s) => Resettable :: Value (s . into ()) , None => Resettable :: Reset , } } }
};
}
