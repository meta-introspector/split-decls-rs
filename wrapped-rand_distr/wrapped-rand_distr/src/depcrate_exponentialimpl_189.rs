// Generated macro for impl_189 (impl)
macro_rules! Depcrate_exponentialimpl_189 {
() => {
// Module: crate::exponential
// Provides: {"impl_189"}
// Dependencies: {}
impl < F : Float > Exp < F > where F : Float , Exp1 : Distribution < F > , { # [doc = " Construct a new `Exp` with the given shape parameter"] # [doc = " `lambda`."] # [doc = ""] # [doc = " # Remarks"] # [doc = ""] # [doc = " For custom types `N` implementing the [`Float`] trait,"] # [doc = " the case `lambda = 0` is handled as follows: each sample corresponds"] # [doc = " to a sample from an `Exp1` multiplied by `1 / 0`. Primitive types"] # [doc = " yield infinity, since `1 / 0 = infinity`."] # [inline] pub fn new (lambda : F) -> Result < Exp < F > , Error > { if ! (lambda >= F :: zero ()) { return Err (Error :: LambdaTooSmall) ; } Ok (Exp { lambda_inverse : F :: one () / lambda , }) } }
};
}
