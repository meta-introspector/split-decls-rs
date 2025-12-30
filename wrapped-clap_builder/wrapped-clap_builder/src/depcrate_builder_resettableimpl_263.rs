// Generated macro for impl_263 (impl)
macro_rules! Depcrate_builder_resettableimpl_263 {
() => {
// Module: crate::builder::resettable
// Provides: {"impl_263"}
// Dependencies: {}
impl IntoResettable < OsStr > for Option < & 'static str > { fn into_resettable (self) -> Resettable < OsStr > { match self { Some (s) => Resettable :: Value (s . into ()) , None => Resettable :: Reset , } } }
};
}
