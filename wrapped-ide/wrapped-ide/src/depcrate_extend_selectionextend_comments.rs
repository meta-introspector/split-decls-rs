// Generated macro for extend_comments (function)
macro_rules! Depcrate_extend_selectionextend_comments {
() => {
// Module: crate::extend_selection
// Provides: {"extend_comments"}
// Dependencies: {}
fn extend_comments (comment : ast :: Comment) -> Option < TextRange > { let prev = adj_comments (& comment , Direction :: Prev) ; let next = adj_comments (& comment , Direction :: Next) ; if prev != next { Some (TextRange :: new (prev . syntax () . text_range () . start () , next . syntax () . text_range () . end ())) } else { None } }
};
}
