// Generated macro for impl_55 (impl)
macro_rules! Depcrate_atomicimpl_55 {
() => {
// Module: crate::atomic
// Provides: {"impl_55"}
// Dependencies: {}
impl < T : ? Sized + Pointable > fmt :: Debug for Owned < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let (raw , tag) = decompose_tag :: < T > (self . data) ; f . debug_struct ("Owned") . field ("raw" , & raw) . field ("tag" , & tag) . finish () } }
};
}
