// Generated macro for use_tree_to_ast (function)
macro_rules! Depcrate_srcuse_tree_to_ast {
() => {
// Module: crate::src
// Provides: {"use_tree_to_ast"}
// Dependencies: {}
# [doc = " Maps a `UseTree` contained in this import back to its AST node."] pub fn use_tree_to_ast (db : & dyn DefDatabase , use_ast_id : AstId < ast :: Use > , index : Idx < ast :: UseTree > ,) -> ast :: UseTree { use_tree_source_map (db , use_ast_id) [index] . clone () }
};
}
