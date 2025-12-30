// Generated macro for impl_310 (impl)
macro_rules! Depcrate_object_tree_editorimpl_310 {
() => {
// Module: crate::object::tree::editor
// Provides: {"impl_310"}
// Dependencies: {}
impl ToComponents for BString { fn to_components (& self) -> impl Iterator < Item = & BStr > { self . split (| b | * b == b'/') . map (Into :: into) } }
};
}
