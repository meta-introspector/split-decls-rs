// Generated macro for pat_node (function)
macro_rules! Depcrate_consteval_testspat_node {
() => {
// Module: crate::consteval::tests
// Provides: {"pat_node"}
// Dependencies: {}
fn pat_node (body_source_map : & BodySourceMap , pat : PatId , db : & TestDB ,) -> Option < InFile < SyntaxNode > > { Some (match body_source_map . pat_syntax (pat) { Ok (sp) => { let root = db . parse_or_expand (sp . file_id) ; sp . map (| ptr | ptr . to_node (& root) . syntax () . clone ()) } Err (SyntheticSyntax) => return None , }) }
};
}
