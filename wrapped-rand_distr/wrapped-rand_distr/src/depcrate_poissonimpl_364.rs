// Generated macro for impl_364 (impl)
macro_rules! Depcrate_poissonimpl_364 {
() => {
// Module: crate::poisson
// Provides: {"impl_364"}
// Dependencies: {}
impl < F > Poisson < F > where F : Float + FloatConst , StandardUniform : Distribution < F > , { # [doc = " Construct a new `Poisson` with the given shape parameter"] # [doc = " `lambda`."] # [doc = ""] # [doc = " The maximum allowed lambda is [MAX_LAMBDA](Self::MAX_LAMBDA)."] pub fn new (lambda : F) -> Result < Poisson < F > , Error > { if ! lambda . is_finite () { return Err (Error :: NonFinite) ; } if ! (lambda > F :: zero ()) { return Err (Error :: ShapeTooSmall) ; } let method = if lambda < F :: from (12.0) . unwrap () { Method :: Knuth (KnuthMethod :: new (lambda)) } else { if lambda > F :: from (Self :: MAX_LAMBDA) . unwrap () { return Err (Error :: ShapeTooLarge) ; } Method :: Rejection (RejectionMethod :: new (lambda)) } ; Ok (Poisson (method)) } # [doc = " The maximum supported value of `lambda`"] # [doc = ""] # [doc = " This value was selected such that"] # [doc = " `MAX_LAMBDA + 1e6 * sqrt(MAX_LAMBDA) < 2^64 - 1`,"] # [doc = " thus ensuring that the probability of sampling a value larger than"] # [doc = " `u64::MAX` is less than 1e-1000."] # [doc = ""] # [doc = " Applying this limit also solves"] # [doc = " [#1312](https://github.com/rust-random/rand/issues/1312)."] pub const MAX_LAMBDA : f64 = 1.844e19 ; }
};
}
