// Generated macro for impl_183 (impl)
macro_rules! Depcrate_vec_drainimpl_183 {
() => {
// Module: crate::vec::drain
// Provides: {"impl_183"}
// Dependencies: {}
impl < T : fmt :: Debug , A : Allocator > fmt :: Debug for Drain < '_ , T , A > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Drain") . field (& self . iter . as_slice ()) . finish () } }
};
}
