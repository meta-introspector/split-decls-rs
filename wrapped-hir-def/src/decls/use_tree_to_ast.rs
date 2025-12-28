macro_rules! deps {
    () => {
        DefDatabase!();
        Use!();
        UseTree!();
    };
}

macro_rules! use_tree_to_ast {
    () => {
        deps!();
        # [doc = " Maps a `UseTree` contained in this import back to its AST node."] pub fn use_tree_to_ast (db : & dyn DefDatabase , use_ast_id : AstId < ast :: Use > , index : Idx < ast :: UseTree > ,) -> ast :: UseTree { use_tree_source_map (db , use_ast_id) [index] . clone () }
    };
}

use_tree_to_ast!()