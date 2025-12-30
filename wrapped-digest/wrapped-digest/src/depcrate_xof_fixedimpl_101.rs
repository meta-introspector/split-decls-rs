// Generated macro for impl_101 (impl)
macro_rules! Depcrate_xof_fixedimpl_101 {
() => {
// Module: crate::xof_fixed
// Provides: {"impl_101"}
// Dependencies: {}
impl < T : ExtendableOutput , S : ArraySize > ExtendableOutput for XofFixedWrapper < T , S > { type Reader = T :: Reader ; fn finalize_xof (self) -> Self :: Reader { self . hash . finalize_xof () } }
};
}
