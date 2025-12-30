// Generated macro for to_components_bstring_ref (function)
macro_rules! Depcrate_tree_utilsto_components_bstring_ref {
() => {
// Module: crate::tree::utils
// Provides: {"to_components_bstring_ref"}
// Dependencies: {}
pub fn to_components_bstring_ref (rela_path : & BString) -> impl Iterator < Item = & BStr > { rela_path . split (| b | * b == b'/') . map (Into :: into) }
};
}
