// Generated macro for impl_850 (impl)
macro_rules! Depcrate_collections_linked_listimpl_850 {
() => {
// Module: crate::collections::linked_list
// Provides: {"impl_850"}
// Dependencies: {}
# [stable (feature = "extract_if" , since = "1.87.0")] impl < T , F , A > fmt :: Debug for ExtractIf < '_ , T , F , A > where T : fmt :: Debug , A : Allocator , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let peek = self . it . map (| node | unsafe { & node . as_ref () . element }) ; f . debug_struct ("ExtractIf") . field ("peek" , & peek) . finish_non_exhaustive () } }
};
}
