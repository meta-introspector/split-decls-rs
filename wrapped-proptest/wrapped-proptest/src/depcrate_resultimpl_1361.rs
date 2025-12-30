// Generated macro for impl_1361 (impl)
macro_rules! Depcrate_resultimpl_1361 {
() => {
// Module: crate::result
// Provides: {"impl_1361"}
// Dependencies: {}
impl < T : Strategy + fmt :: Debug , E : Strategy + fmt :: Debug > fmt :: Debug for MaybeErr < T , E > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "MaybeErr({:?})" , self . 0) } }
};
}
