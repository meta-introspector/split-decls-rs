// Generated macro for impl_295 (impl)
macro_rules! Depcrate_hybrid_errorimpl_295 {
() => {
// Module: crate::hybrid::error
// Provides: {"impl_295"}
// Dependencies: {}
impl core :: fmt :: Display for BuildError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self . kind { BuildErrorKind :: NFA (_) => write ! (f , "error building NFA") , BuildErrorKind :: InsufficientCacheCapacity { minimum , given } => { write ! (f , "given cache capacity ({}) is smaller than \
                     minimum required ({})" , given , minimum ,) } BuildErrorKind :: InsufficientStateIDCapacity { ref err } => { err . fmt (f) } BuildErrorKind :: Unsupported (ref msg) => { write ! (f , "unsupported regex feature for DFAs: {}" , msg) } } } }
};
}
