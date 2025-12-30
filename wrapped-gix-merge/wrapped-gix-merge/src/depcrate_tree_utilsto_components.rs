// Generated macro for to_components (function)
macro_rules! Depcrate_tree_utilsto_components {
() => {
// Module: crate::tree::utils
// Provides: {"to_components"}
// Dependencies: {}
pub fn to_components (rela_path : & BStr) -> impl Iterator < Item = & BStr > { rela_path . split (| b | * b == b'/') . map (Into :: into) }
};
}
