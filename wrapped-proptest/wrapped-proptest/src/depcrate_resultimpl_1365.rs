// Generated macro for impl_1365 (impl)
macro_rules! Depcrate_resultimpl_1365 {
() => {
// Module: crate::result
// Provides: {"impl_1365"}
// Dependencies: {}
impl < T : Strategy , E : Strategy > fmt :: Debug for MaybeErrValueTree < T , E > where T :: Tree : fmt :: Debug , E :: Tree : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "MaybeErrValueTree({:?})" , self . 0) } }
};
}
