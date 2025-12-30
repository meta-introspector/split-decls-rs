// Generated macro for impl_39 (impl)
macro_rules! Depcrate_atomicimpl_39 {
() => {
// Module: crate::atomic
// Provides: {"impl_39"}
// Dependencies: {}
impl < T : ? Sized + Pointable > fmt :: Debug for Atomic < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let data = self . data . load (Ordering :: SeqCst) ; let (raw , tag) = decompose_tag :: < T > (data) ; f . debug_struct ("Atomic") . field ("raw" , & raw) . field ("tag" , & tag) . finish () } }
};
}
