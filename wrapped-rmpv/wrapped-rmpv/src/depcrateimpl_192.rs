// Generated macro for impl_192 (impl)
macro_rules! Depcrateimpl_192 {
() => {
// Module: crate
// Provides: {"impl_192"}
// Dependencies: {}
impl Display for Utf8StringRef < '_ > { # [cold] fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { match self . s { Ok (s) => Debug :: fmt (& & s , fmt) , Err (ref err) => Debug :: fmt (& err . 0 , fmt) , } } }
};
}
