// Generated macro for impl_348 (impl)
macro_rules! Depcrate_pertimpl_348 {
() => {
// Module: crate::pert
// Provides: {"impl_348"}
// Dependencies: {}
impl < F > Distribution < F > for Pert < F > where F : Float , StandardNormal : Distribution < F > , Exp1 : Distribution < F > , Open01 : Distribution < F > , { # [inline] fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> F { self . beta . sample (rng) * self . range + self . min } }
};
}
