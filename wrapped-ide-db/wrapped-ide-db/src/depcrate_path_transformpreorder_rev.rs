// Generated macro for preorder_rev (function)
macro_rules! Depcrate_path_transformpreorder_rev {
() => {
// Module: crate::path_transform
// Provides: {"preorder_rev"}
// Dependencies: {}
fn preorder_rev (item : & SyntaxNode) -> impl Iterator < Item = SyntaxNode > { let x = item . preorder () . filter_map (| event | match event { syntax :: WalkEvent :: Enter (node) => Some (node) , syntax :: WalkEvent :: Leave (_) => None , }) . collect_vec () ; x . into_iter () . rev () }
};
}
