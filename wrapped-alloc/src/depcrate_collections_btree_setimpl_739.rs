// Generated macro for impl_739 (impl)
macro_rules! Depcrate_collections_btree_setimpl_739 {
() => {
// Module: crate::collections::btree::set
// Provides: {"impl_739"}
// Dependencies: {}
# [stable (feature = "btree_extract_if" , since = "1.91.0")] impl < T , R , F , A > fmt :: Debug for ExtractIf < '_ , T , R , F , A > where T : fmt :: Debug , A : Allocator + Clone , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ExtractIf") . field ("peek" , & self . inner . peek () . map (| (k , _) | k)) . finish_non_exhaustive () } }
};
}
