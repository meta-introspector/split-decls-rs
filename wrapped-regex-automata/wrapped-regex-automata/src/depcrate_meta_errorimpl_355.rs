// Generated macro for impl_355 (impl)
macro_rules! Depcrate_meta_errorimpl_355 {
() => {
// Module: crate::meta::error
// Provides: {"impl_355"}
// Dependencies: {}
impl core :: fmt :: Display for RetryError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match * self { RetryError :: Quadratic (ref err) => err . fmt (f) , RetryError :: Fail (ref err) => err . fmt (f) , } } }
};
}
