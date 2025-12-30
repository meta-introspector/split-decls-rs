// Generated macro for contiguous_range_for_item_group (function)
macro_rules! Depcrate_folding_rangescontiguous_range_for_item_group {
() => {
// Module: crate::folding_ranges
// Provides: {"contiguous_range_for_item_group"}
// Dependencies: {}
fn contiguous_range_for_item_group < N > (first : N , visited : & mut FxHashSet < SyntaxNode > ,) -> Option < TextRange > where N : ast :: HasVisibility + Clone + Hash + Eq , { if ! visited . insert (first . syntax () . clone ()) { return None ; } let (mut last , mut last_vis) = (first . clone () , first . visibility ()) ; for element in first . syntax () . siblings_with_tokens (Direction :: Next) { let node = match element { NodeOrToken :: Token (token) => { if let Some (ws) = ast :: Whitespace :: cast (token) && ! ws . spans_multiple_lines () { continue ; } break ; } NodeOrToken :: Node (node) => node , } ; if let Some (next) = N :: cast (node) { let next_vis = next . visibility () ; if eq_visibility (next_vis . clone () , last_vis) { visited . insert (next . syntax () . clone ()) ; last_vis = next_vis ; last = next ; continue ; } } break ; } if first != last { Some (TextRange :: new (first . syntax () . text_range () . start () , last . syntax () . text_range () . end ())) } else { None } }
};
}
