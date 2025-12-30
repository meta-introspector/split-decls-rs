// Generated macro for impl_522 (impl)
macro_rules! Depcrate_setimpl_522 {
() => {
// Module: crate::set
// Provides: {"impl_522"}
// Dependencies: {}
impl < K : fmt :: Debug , A : Allocator > fmt :: Debug for IntoIter < K , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let entries_iter = self . iter . iter () . map (| (k , _) | k) ; f . debug_list () . entries (entries_iter) . finish () } }
};
}
