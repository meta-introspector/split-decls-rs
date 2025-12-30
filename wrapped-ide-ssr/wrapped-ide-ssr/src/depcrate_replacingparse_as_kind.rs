// Generated macro for parse_as_kind (function)
macro_rules! Depcrate_replacingparse_as_kind {
() => {
// Module: crate::replacing
// Provides: {"parse_as_kind"}
// Dependencies: {}
fn parse_as_kind (code : & str , kind : SyntaxKind) -> Option < SyntaxNode > { if ast :: Expr :: can_cast (kind) && let Ok (expr) = fragments :: expr (code) { return Some (expr) ; } if ast :: Item :: can_cast (kind) && let Ok (item) = fragments :: item (code) { return Some (item) ; } None }
};
}
