// Generated macro for impl_135 (impl)
macro_rules! Depcrate_taskimpl_135 {
() => {
// Module: crate::task
// Provides: {"impl_135"}
// Dependencies: {}
impl < T , M : fmt :: Debug > fmt :: Debug for FallibleTask < T , M > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("FallibleTask") . field ("header" , self . task . header_with_metadata ()) . finish () } }
};
}
