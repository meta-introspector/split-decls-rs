// Generated macro for impl_307 (impl)
macro_rules! Depcrate_object_tree_editorimpl_307 {
() => {
// Module: crate::object::tree::editor
// Provides: {"impl_307"}
// Dependencies: {}
impl ToComponents for & str { fn to_components (& self) -> impl Iterator < Item = & BStr > { self . split ('/') . map (Into :: into) } }
};
}
