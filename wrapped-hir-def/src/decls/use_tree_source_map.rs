macro_rules! deps {
    () => {
        UseTree!();
        DefDatabase!();
        Use!();
    };
}

macro_rules! use_tree_source_map {
    () => {
        deps!();
        # [doc = " Maps a `UseTree` contained in this import back to its AST node."] fn use_tree_source_map (db : & dyn DefDatabase , use_ast_id : AstId < ast :: Use >) -> Arena < ast :: UseTree > { let ast = use_ast_id . to_node (db) ; let ast_use_tree = ast . use_tree () . expect ("missing `use_tree`") ; let mut span_map = None ; crate :: item_tree :: lower_use_tree (db , ast_use_tree , & mut | range | { span_map . get_or_insert_with (| | db . span_map (use_ast_id . file_id)) . span_for_range (range) . ctx }) . expect ("failed to lower use tree") . 1 }
    };
}

use_tree_source_map!();