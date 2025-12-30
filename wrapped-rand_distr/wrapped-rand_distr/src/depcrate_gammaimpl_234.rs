// Generated macro for impl_234 (impl)
macro_rules! Depcrate_gammaimpl_234 {
() => {
// Module: crate::gamma
// Provides: {"impl_234"}
// Dependencies: {}
impl < F > Distribution < F > for Gamma < F > where F : Float , StandardNormal : Distribution < F > , Exp1 : Distribution < F > , Open01 : Distribution < F > , { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> F { match self . repr { Small (ref g) => g . sample (rng) , One (ref g) => g . sample (rng) , Large (ref g) => g . sample (rng) , } } }
};
}
