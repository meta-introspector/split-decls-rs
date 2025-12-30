// Generated macro for impl_259 (impl)
macro_rules! Depcrate_builder_resettableimpl_259 {
() => {
// Module: crate::builder::resettable
// Provides: {"impl_259"}
// Dependencies: {}
impl IntoResettable < ArgAction > for Option < ArgAction > { fn into_resettable (self) -> Resettable < ArgAction > { match self { Some (s) => Resettable :: Value (s) , None => Resettable :: Reset , } } }
};
}
