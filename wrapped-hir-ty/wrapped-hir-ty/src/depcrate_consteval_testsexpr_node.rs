// Generated macro for expr_node (function)
macro_rules! Depcrate_consteval_testsexpr_node {
() => {
// Module: crate::consteval::tests
// Provides: {"expr_node"}
// Dependencies: {}
fn expr_node (body_source_map : & BodySourceMap , expr : ExprId , db : & TestDB ,) -> Option < InFile < SyntaxNode > > { Some (match body_source_map . expr_syntax (expr) { Ok (sp) => { let root = db . parse_or_expand (sp . file_id) ; sp . map (| ptr | ptr . to_node (& root) . syntax () . clone ()) } Err (SyntheticSyntax) => return None , }) }
};
}
