// Generated macro for impl_531 (impl)
macro_rules! Depcrateimpl_531 {
() => {
// Module: crate
// Provides: {"impl_531"}
// Dependencies: {}
impl fmt :: Display for ToCompactStringError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { ToCompactStringError :: Reserve (err) => err . fmt (f) , ToCompactStringError :: Fmt (err) => err . fmt (f) , } } }
};
}
