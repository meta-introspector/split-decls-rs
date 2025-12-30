// Generated macro for impl_88 (impl)
macro_rules! Depcrate_slice_revimpl_88 {
() => {
// Module: crate::slice::rev
// Provides: {"impl_88"}
// Dependencies: {}
impl < T > SliceFind for RevSlice < T > { type Item = T ; fn find < U : ? Sized > (& self , elt : & U) -> Option < usize > where Self :: Item : PartialEq < U > , { self . 0 . rfind (elt) . map (move | i | self . raw_index_no_wrap (i)) } fn rfind < U : ? Sized > (& self , elt : & U) -> Option < usize > where Self :: Item : PartialEq < U > , { self . 0 . find (elt) . map (move | i | self . raw_index_no_wrap (i)) } }
};
}
