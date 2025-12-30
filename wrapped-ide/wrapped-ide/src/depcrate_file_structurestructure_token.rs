// Generated macro for structure_token (function)
macro_rules! Depcrate_file_structurestructure_token {
() => {
// Module: crate::file_structure
// Provides: {"structure_token"}
// Dependencies: {}
fn structure_token (token : SyntaxToken) -> Option < StructureNode > { if let Some (comment) = ast :: Comment :: cast (token) { let text = comment . text () . trim () ; if let Some (region_name) = text . strip_prefix ("// region:") . map (str :: trim) . filter (| it | ! it . is_empty ()) { return Some (StructureNode { parent : None , label : region_name . to_owned () , navigation_range : comment . syntax () . text_range () , node_range : comment . syntax () . text_range () , kind : StructureNodeKind :: Region , detail : None , deprecated : false , }) ; } } None }
};
}
