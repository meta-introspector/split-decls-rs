// Generated macro for impl_312 (impl)
macro_rules! Depcrate_object_tree_editorimpl_312 {
() => {
// Module: crate::object::tree::editor
// Provides: {"impl_312"}
// Dependencies: {}
impl ToComponents for & BStr { fn to_components (& self) -> impl Iterator < Item = & BStr > { self . split (| b | * b == b'/') . map (Into :: into) } }
};
}
