// Generated macro for impl_88 (impl)
macro_rules! Depcrate_xof_fixedimpl_88 {
() => {
// Module: crate::xof_fixed
// Provides: {"impl_88"}
// Dependencies: {}
impl < T : ExtendableOutput + fmt :: Debug , S : ArraySize > fmt :: Debug for XofFixedWrapper < T , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("XofFixedWrapper") . field ("hash" , & self . hash) . field ("_size" , & self . size) . finish () } }
};
}
