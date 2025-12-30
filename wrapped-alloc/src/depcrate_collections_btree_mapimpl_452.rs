// Generated macro for impl_452 (impl)
macro_rules! Depcrate_collections_btree_mapimpl_452 {
() => {
// Module: crate::collections::btree::map
// Provides: {"impl_452"}
// Dependencies: {}
# [stable (feature = "btree_extract_if" , since = "1.91.0")] impl < K , V , R , F , A > fmt :: Debug for ExtractIf < '_ , K , V , R , F , A > where K : fmt :: Debug , V : fmt :: Debug , A : Allocator + Clone , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ExtractIf") . field ("peek" , & self . inner . peek ()) . finish_non_exhaustive () } }
};
}
