// Generated macro for impl_203 (impl)
macro_rules! Depcrate_fisher_fimpl_203 {
() => {
// Module: crate::fisher_f
// Provides: {"impl_203"}
// Dependencies: {}
impl < F > Distribution < F > for FisherF < F > where F : Float , StandardNormal : Distribution < F > , Exp1 : Distribution < F > , Open01 : Distribution < F > , { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> F { self . numer . sample (rng) / self . denom . sample (rng) * self . dof_ratio } }
};
}
