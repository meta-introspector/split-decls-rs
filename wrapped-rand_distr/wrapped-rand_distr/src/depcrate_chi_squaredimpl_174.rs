// Generated macro for impl_174 (impl)
macro_rules! Depcrate_chi_squaredimpl_174 {
() => {
// Module: crate::chi_squared
// Provides: {"impl_174"}
// Dependencies: {}
impl < F > Distribution < F > for ChiSquared < F > where F : Float , StandardNormal : Distribution < F > , Exp1 : Distribution < F > , Open01 : Distribution < F > , { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> F { match self . repr { DoFExactlyOne => { let norm : F = rng . sample (StandardNormal) ; norm * norm } DoFAnythingElse (ref g) => g . sample (rng) , } } }
};
}
