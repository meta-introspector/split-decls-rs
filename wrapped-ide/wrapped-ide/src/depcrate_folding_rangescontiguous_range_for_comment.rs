// Generated macro for contiguous_range_for_comment (function)
macro_rules! Depcrate_folding_rangescontiguous_range_for_comment {
() => {
// Module: crate::folding_ranges
// Provides: {"contiguous_range_for_comment"}
// Dependencies: {}
fn contiguous_range_for_comment (first : ast :: Comment , visited : & mut FxHashSet < ast :: Comment > ,) -> Option < TextRange > { visited . insert (first . clone ()) ; let group_kind = first . kind () ; if ! group_kind . shape . is_line () { return None ; } let mut last = first . clone () ; for element in first . syntax () . siblings_with_tokens (Direction :: Next) { match element { NodeOrToken :: Token (token) => { if let Some (ws) = ast :: Whitespace :: cast (token . clone ()) && ! ws . spans_multiple_lines () { continue ; } if let Some (c) = ast :: Comment :: cast (token) && c . kind () == group_kind { let text = c . text () . trim_start () ; if ! (text . starts_with (REGION_START) || text . starts_with (REGION_END)) { visited . insert (c . clone ()) ; last = c ; continue ; } } break ; } NodeOrToken :: Node (_) => break , } ; } if first != last { Some (TextRange :: new (first . syntax () . text_range () . start () , last . syntax () . text_range () . end ())) } else { None } }
};
}
