// Generated macro for impl_446 (impl)
macro_rules! Depcrate_weibullimpl_446 {
() => {
// Module: crate::weibull
// Provides: {"impl_446"}
// Dependencies: {}
impl < F > Weibull < F > where F : Float , OpenClosed01 : Distribution < F > , { # [doc = " Construct a new `Weibull` distribution with given `scale` and `shape`."] pub fn new (scale : F , shape : F) -> Result < Weibull < F > , Error > { if ! (scale > F :: zero ()) { return Err (Error :: ScaleTooSmall) ; } if ! (shape > F :: zero ()) { return Err (Error :: ShapeTooSmall) ; } Ok (Weibull { inv_shape : F :: from (1.) . unwrap () / shape , scale , }) } }
};
}
