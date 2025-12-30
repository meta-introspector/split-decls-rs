// Generated macro for impl_1933 (impl)
macro_rules! Depcrate_entryimpl_1933 {
() => {
// Module: crate::entry
// Provides: {"impl_1933"}
// Dependencies: {}
impl MapType { fn name (self) -> & 'static str { match self { Self :: Hash => "HashMap" , Self :: BTree => "BTreeMap" , } } fn entry_path (self) -> & 'static str { match self { Self :: Hash => "std::collections::hash_map::Entry" , Self :: BTree => "std::collections::btree_map::Entry" , } } }
};
}
