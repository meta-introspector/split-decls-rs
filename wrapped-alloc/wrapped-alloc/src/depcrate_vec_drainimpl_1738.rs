// Generated macro for impl_1738 (impl)
macro_rules! Depcrate_vec_drainimpl_1738 {
() => {
// Module: crate::vec::drain
// Provides: {"impl_1738"}
// Dependencies: {}
# [stable (feature = "collection_debug" , since = "1.17.0")] impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for Drain < '_ , T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Drain") . field (& self . iter . as_slice ()) . finish () } }
};
}
