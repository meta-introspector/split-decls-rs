// Generated macro for span_of_first_expr_in_block (function)
macro_rules! Depcrate_needless_continuespan_of_first_expr_in_block {
() => {
// Module: crate::needless_continue
// Provides: {"span_of_first_expr_in_block"}
// Dependencies: {}
fn span_of_first_expr_in_block (block : & Block < '_ >) -> Option < Span > { block . stmts . first () . map (| stmt | stmt . span) . or (block . expr . map (| expr | expr . span)) }
};
}
