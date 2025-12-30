// Generated macro for impl_1715 (impl)
macro_rules! Depcrate_vec_extract_ifimpl_1715 {
() => {
// Module: crate::vec::extract_if
// Provides: {"impl_1715"}
// Dependencies: {}
# [stable (feature = "extract_if" , since = "1.87.0")] impl < T , F , A > fmt :: Debug for ExtractIf < '_ , T , F , A > where T : fmt :: Debug , A : Allocator , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let peek = if self . idx < self . end { self . vec . get (self . idx) } else { None } ; f . debug_struct ("ExtractIf") . field ("peek" , & peek) . finish_non_exhaustive () } }
};
}
