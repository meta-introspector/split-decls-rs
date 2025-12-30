// Generated macro for impl_526 (impl)
macro_rules! Depcrate_setimpl_526 {
() => {
// Module: crate::set
// Provides: {"impl_526"}
// Dependencies: {}
impl < K : fmt :: Debug , A : Allocator > fmt :: Debug for Drain < '_ , K , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let entries_iter = self . iter . iter () . map (| (k , _) | k) ; f . debug_list () . entries (entries_iter) . finish () } }
};
}
