// Generated macro for impl_87 (impl)
macro_rules! Depcrate_xof_fixedimpl_87 {
() => {
// Module: crate::xof_fixed
// Provides: {"impl_87"}
// Dependencies: {}
impl < T : ExtendableOutput + Clone , S : ArraySize > Clone for XofFixedWrapper < T , S > { fn clone (& self) -> Self { Self { hash : self . hash . clone () , size : PhantomData , } } }
};
}
