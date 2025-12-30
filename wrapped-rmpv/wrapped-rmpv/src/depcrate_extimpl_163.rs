// Generated macro for impl_163 (impl)
macro_rules! Depcrate_extimpl_163 {
() => {
// Module: crate::ext
// Provides: {"impl_163"}
// Dependencies: {}
impl Display for Error { # [cold] fn fmt (& self , fmt : & mut Formatter < '_ >) -> Result < () , fmt :: Error > { match * self { Self :: Syntax (ref err) => write ! (fmt , "error while decoding value: {err}") , } } }
};
}
