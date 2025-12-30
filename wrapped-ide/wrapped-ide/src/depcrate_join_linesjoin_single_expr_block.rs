// Generated macro for join_single_expr_block (function)
macro_rules! Depcrate_join_linesjoin_single_expr_block {
() => {
// Module: crate::join_lines
// Provides: {"join_single_expr_block"}
// Dependencies: {}
fn join_single_expr_block (edit : & mut TextEditBuilder , token : & SyntaxToken) -> Option < () > { let block_expr = ast :: BlockExpr :: cast (token . parent_ancestors () . nth (1) ?) ? ; if ! block_expr . is_standalone () { return None ; } let expr = extract_trivial_expression (& block_expr) ? ; let block_range = block_expr . syntax () . text_range () ; let mut buf = expr . syntax () . text () . to_string () ; if let Some (match_arm) = block_expr . syntax () . parent () . and_then (ast :: MatchArm :: cast) && match_arm . comma_token () . is_none () { buf . push (',') ; } edit . replace (block_range , buf) ; Some (()) }
};
}
