// Generated macro for impl_1935 (impl)
macro_rules! Depcrate_r2d2impl_1935 {
() => {
// Module: crate::r2d2
// Provides: {"impl_1935"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Error :: ConnectionError (ref e) => e . fmt (f) , Error :: QueryError (ref e) => e . fmt (f) , } } }
};
}
