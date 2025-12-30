// Generated macro for impl_130 (impl)
macro_rules! Depcrate_store_impls_dynamic_load_indeximpl_130 {
() => {
// Module: crate::store_impls::dynamic::load_index
// Provides: {"impl_130"}
// Dependencies: {}
impl Either { fn path (& self) -> & Path { match self { Either :: IndexPath (p) => p , Either :: MultiIndexFile (f) => f . path () , } } fn into_index_and_packs (self , mtime : SystemTime) -> IndexAndPacks { match self { Either :: IndexPath (path) => IndexAndPacks :: new_single (path , mtime) , Either :: MultiIndexFile (file) => IndexAndPacks :: new_multi_from_open_file (file , mtime) , } } fn is_multi_index (& self) -> bool { matches ! (self , Either :: MultiIndexFile (_)) } }
};
}
