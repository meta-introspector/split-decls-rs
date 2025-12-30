// Generated macro for impl_190 (impl)
macro_rules! Depcrate_exponentialimpl_190 {
() => {
// Module: crate::exponential
// Provides: {"impl_190"}
// Dependencies: {}
impl < F > Distribution < F > for Exp < F > where F : Float , Exp1 : Distribution < F > , { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> F { rng . sample (Exp1) * self . lambda_inverse } }
};
}
