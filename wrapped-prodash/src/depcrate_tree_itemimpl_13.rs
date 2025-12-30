// Generated macro for impl_13 (impl)
macro_rules! Depcrate_tree_itemimpl_13 {
() => {
// Module: crate::tree::item
// Provides: {"impl_13"}
// Dependencies: {}
impl Debug for Item { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("Item") . field ("key" , & self . key) . field ("value" , & self . value) . finish_non_exhaustive () } }
};
}
