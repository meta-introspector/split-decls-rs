// Generated macro for impl_313 (impl)
macro_rules! Depcrate_hybrid_idimpl_313 {
() => {
// Module: crate::hybrid::id
// Provides: {"impl_313"}
// Dependencies: {}
impl core :: fmt :: Display for LazyStateIDError { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "failed to create LazyStateID from {:?}, which exceeds {:?}" , self . attempted () , LazyStateID :: MAX ,) } }
};
}
