// Generated macro for impl_214 (impl)
macro_rules! Depcrate_frechetimpl_214 {
() => {
// Module: crate::frechet
// Provides: {"impl_214"}
// Dependencies: {}
impl < F > Frechet < F > where F : Float , OpenClosed01 : Distribution < F > , { # [doc = " Construct a new `Frechet` distribution with given `location`, `scale`, and `shape`."] pub fn new (location : F , scale : F , shape : F) -> Result < Frechet < F > , Error > { if scale <= F :: zero () || scale . is_infinite () || scale . is_nan () { return Err (Error :: ScaleNotPositive) ; } if shape <= F :: zero () || shape . is_infinite () || shape . is_nan () { return Err (Error :: ShapeNotPositive) ; } if location . is_infinite () || location . is_nan () { return Err (Error :: LocationNotFinite) ; } Ok (Frechet { location , scale , shape , }) } }
};
}
