// Generated macro for fold_range_for_multiline_match_arm (function)
macro_rules! Depcrate_folding_rangesfold_range_for_multiline_match_arm {
() => {
// Module: crate::folding_ranges
// Provides: {"fold_range_for_multiline_match_arm"}
// Dependencies: {}
fn fold_range_for_multiline_match_arm (match_arm : ast :: MatchArm) -> Option < TextRange > { if fold_kind (match_arm . expr () ? . syntax () . kind ()) . is_some () { None } else if match_arm . expr () ? . syntax () . text () . contains_char ('\n') { Some (match_arm . expr () ? . syntax () . text_range ()) } else { None } }
};
}
