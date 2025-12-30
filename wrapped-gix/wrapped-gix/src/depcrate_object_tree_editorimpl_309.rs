// Generated macro for impl_309 (impl)
macro_rules! Depcrate_object_tree_editorimpl_309 {
() => {
// Module: crate::object::tree::editor
// Provides: {"impl_309"}
// Dependencies: {}
impl ToComponents for & String { fn to_components (& self) -> impl Iterator < Item = & BStr > { self . split ('/') . map (Into :: into) } }
};
}
