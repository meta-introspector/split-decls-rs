// Generated macro for impl_341 (impl)
macro_rules! Depcrate_streamimpl_341 {
() => {
// Module: crate::stream
// Provides: {"impl_341"}
// Dependencies: {}
impl < T > fmt :: Display for PointerOffset < T > where T : ? Sized , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "PointerOffset({:?})" , self . 0 as * const ()) } }
};
}
