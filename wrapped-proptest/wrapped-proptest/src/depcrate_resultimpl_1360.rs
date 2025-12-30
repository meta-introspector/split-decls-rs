// Generated macro for impl_1360 (impl)
macro_rules! Depcrate_resultimpl_1360 {
() => {
// Module: crate::result
// Provides: {"impl_1360"}
// Dependencies: {}
impl < T : Strategy + fmt :: Debug , E : Strategy + fmt :: Debug > fmt :: Debug for MaybeOk < T , E > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "MaybeOk({:?})" , self . 0) } }
};
}
