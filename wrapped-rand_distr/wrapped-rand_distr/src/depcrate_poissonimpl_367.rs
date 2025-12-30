// Generated macro for impl_367 (impl)
macro_rules! Depcrate_poissonimpl_367 {
() => {
// Module: crate::poisson
// Provides: {"impl_367"}
// Dependencies: {}
impl < F > Distribution < F > for Poisson < F > where F : Float + FloatConst , StandardUniform : Distribution < F > , StandardNormal : Distribution < F > , Exp1 : Distribution < F > , { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> F { match & self . 0 { Method :: Knuth (method) => method . sample (rng) , Method :: Rejection (method) => method . sample (rng) , } } }
};
}
