// Generated macro for impl_186 (impl)
macro_rules! Depcrateimpl_186 {
() => {
// Module: crate
// Provides: {"impl_186"}
// Dependencies: {}
impl Display for Utf8String { # [cold] fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { match self . s { Ok (ref s) => Debug :: fmt (& s , fmt) , Err (ref err) => Debug :: fmt (& err . 0 , fmt) , } } }
};
}
