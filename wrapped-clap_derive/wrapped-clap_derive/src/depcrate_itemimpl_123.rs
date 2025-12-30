// Generated macro for impl_123 (impl)
macro_rules! Depcrate_itemimpl_123 {
() => {
// Module: crate::item
// Provides: {"impl_123"}
// Dependencies: {}
impl Deprecation { fn attribute (version : & 'static str , old : AttrKind , new : AttrKind , span : Span) -> Self { Self { span , id : "old_attribute" , version , description : format ! ("Attribute `#[{}(...)]` has been deprecated in favor of `#[{}(...)]`" , old . as_str () , new . as_str ()) , } } }
};
}
