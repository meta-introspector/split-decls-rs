// Generated macro for impl_77 (impl)
macro_rules! Depcrate_atomicimpl_77 {
() => {
// Module: crate::atomic
// Provides: {"impl_77"}
// Dependencies: {}
impl < T : ? Sized + Pointable > fmt :: Debug for Shared < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let (raw , tag) = decompose_tag :: < T > (self . data) ; f . debug_struct ("Shared") . field ("raw" , & raw) . field ("tag" , & tag) . finish () } }
};
}
