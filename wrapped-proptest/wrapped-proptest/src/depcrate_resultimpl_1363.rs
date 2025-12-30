// Generated macro for impl_1363 (impl)
macro_rules! Depcrate_resultimpl_1363 {
() => {
// Module: crate::result
// Provides: {"impl_1363"}
// Dependencies: {}
impl < T : Strategy , E : Strategy > fmt :: Debug for MaybeOkValueTree < T , E > where T :: Tree : fmt :: Debug , E :: Tree : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "MaybeOkValueTree({:?})" , self . 0) } }
};
}
