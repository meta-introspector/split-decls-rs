// Generated macro for impl_173 (impl)
macro_rules! Depcrate_chi_squaredimpl_173 {
() => {
// Module: crate::chi_squared
// Provides: {"impl_173"}
// Dependencies: {}
impl < F > ChiSquared < F > where F : Float , StandardNormal : Distribution < F > , Exp1 : Distribution < F > , Open01 : Distribution < F > , { # [doc = " Create a new chi-squared distribution with degrees-of-freedom"] # [doc = " `k`."] pub fn new (k : F) -> Result < ChiSquared < F > , Error > { let repr = if k == F :: one () { DoFExactlyOne } else { if ! (F :: from (0.5) . unwrap () * k > F :: zero ()) { return Err (Error :: DoFTooSmall) ; } DoFAnythingElse (Gamma :: new (F :: from (0.5) . unwrap () * k , F :: from (2.0) . unwrap ()) . unwrap ()) } ; Ok (ChiSquared { repr }) } }
};
}
