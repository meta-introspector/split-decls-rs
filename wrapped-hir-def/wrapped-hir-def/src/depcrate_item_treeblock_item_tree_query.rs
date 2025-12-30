// Generated macro for block_item_tree_query (function)
macro_rules! Depcrate_item_treeblock_item_tree_query {
() => {
// Module: crate::item_tree
// Provides: {"block_item_tree_query"}
// Dependencies: {}
# [salsa_macros :: tracked (returns (deref))] pub (crate) fn block_item_tree_query (db : & dyn DefDatabase , block : BlockId) -> Arc < ItemTree > { let _p = tracing :: info_span ! ("block_item_tree_query" , ? block) . entered () ; static EMPTY : OnceLock < Arc < ItemTree > > = OnceLock :: new () ; let loc = block . lookup (db) ; let block = loc . ast_id . to_node (db) ; let ctx = lower :: Ctx :: new (db , loc . ast_id . file_id) ; let mut item_tree = ctx . lower_block (& block) ; let ItemTree { top_level , top_attrs , attrs , vis , big_data , small_data } = & item_tree ; if small_data . is_empty () && big_data . is_empty () && top_level . is_empty () && attrs . is_empty () && top_attrs . is_empty () && vis . arena . is_empty () { EMPTY . get_or_init (| | { Arc :: new (ItemTree { top_level : Box :: new ([]) , attrs : FxHashMap :: default () , small_data : FxHashMap :: default () , big_data : FxHashMap :: default () , top_attrs : RawAttrs :: EMPTY , vis : ItemVisibilities { arena : ThinVec :: new () } , }) }) . clone () } else { item_tree . shrink_to_fit () ; Arc :: new (item_tree) } }
};
}
