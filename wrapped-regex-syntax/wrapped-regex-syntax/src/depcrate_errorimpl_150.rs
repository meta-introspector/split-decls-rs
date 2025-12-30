// Generated macro for impl_150 (impl)
macro_rules! Depcrate_errorimpl_150 {
() => {
// Module: crate::error
// Provides: {"impl_150"}
// Dependencies: {}
impl core :: fmt :: Display for Error { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match * self { Error :: Parse (ref x) => x . fmt (f) , Error :: Translate (ref x) => x . fmt (f) , } } }
};
}
