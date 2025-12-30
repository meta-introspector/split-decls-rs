// Generated macro for impl_56 (impl)
macro_rules! Depcrate_atomic_atomic_cellimpl_56 {
() => {
// Module: crate::atomic::atomic_cell
// Provides: {"impl_56"}
// Dependencies: {}
impl < T : Copy + fmt :: Debug > fmt :: Debug for AtomicCell < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("AtomicCell") . field ("value" , & self . load ()) . finish () } }
};
}
