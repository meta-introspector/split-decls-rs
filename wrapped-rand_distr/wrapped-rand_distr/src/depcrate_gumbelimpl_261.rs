// Generated macro for impl_261 (impl)
macro_rules! Depcrate_gumbelimpl_261 {
() => {
// Module: crate::gumbel
// Provides: {"impl_261"}
// Dependencies: {}
impl < F > Gumbel < F > where F : Float , OpenClosed01 : Distribution < F > , { # [doc = " Construct a new `Gumbel` distribution with given `location` and `scale`."] pub fn new (location : F , scale : F) -> Result < Gumbel < F > , Error > { if scale <= F :: zero () || scale . is_infinite () || scale . is_nan () { return Err (Error :: ScaleNotPositive) ; } if location . is_infinite () || location . is_nan () { return Err (Error :: LocationNotFinite) ; } Ok (Gumbel { location , scale }) } }
};
}
