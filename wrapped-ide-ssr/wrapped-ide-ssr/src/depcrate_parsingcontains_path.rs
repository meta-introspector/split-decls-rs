// Generated macro for contains_path (function)
macro_rules! Depcrate_parsingcontains_path {
() => {
// Module: crate::parsing
// Provides: {"contains_path"}
// Dependencies: {}
# [doc = " Returns whether there are any paths in `node`."] fn contains_path (node : & SyntaxNode) -> bool { node . kind () == SyntaxKind :: PATH || node . descendants () . any (| node | node . kind () == SyntaxKind :: PATH) }
};
}
