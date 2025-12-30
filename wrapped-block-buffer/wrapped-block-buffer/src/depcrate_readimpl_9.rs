// Generated macro for impl_9 (impl)
macro_rules! Depcrate_readimpl_9 {
() => {
// Module: crate::read
// Provides: {"impl_9"}
// Dependencies: {}
impl < BS : ArraySize > fmt :: Debug for ReadBuffer < BS > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ReadBuffer") . field ("remaining_data" , & self . remaining ()) . finish_non_exhaustive () } }
};
}
