// Generated macro for impl_231 (impl)
macro_rules! Depcrate_item_treeimpl_231 {
() => {
// Module: crate::item_tree
// Provides: {"impl_231"}
// Dependencies: {}
impl TreeId { pub (crate) fn new (file : HirFileId , block : Option < BlockId >) -> Self { Self { file , block } } pub (crate) fn item_tree < 'db > (& self , db : & 'db dyn DefDatabase) -> & 'db ItemTree { match self . block { Some (block) => block_item_tree_query (db , block) , None => file_item_tree_query (db , self . file) , } } # [inline] pub fn file_id (self) -> HirFileId { self . file } pub (crate) fn is_block (self) -> bool { self . block . is_some () } }
};
}
