// Generated macro for impl_327 (impl)
macro_rules! Depcrate_vec_drainimpl_327 {
() => {
// Module: crate::vec::drain
// Provides: {"impl_327"}
// Dependencies: {}
impl < T : fmt :: Debug , LenT : LenType > fmt :: Debug for Drain < '_ , T , LenT > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Drain") . field (& self . iter . as_slice ()) . finish () } }
};
}
