// Generated macro for impl_389 (impl)
macro_rules! Depcrate_student_timpl_389 {
() => {
// Module: crate::student_t
// Provides: {"impl_389"}
// Dependencies: {}
impl < F > Distribution < F > for StudentT < F > where F : Float , StandardNormal : Distribution < F > , Exp1 : Distribution < F > , Open01 : Distribution < F > , { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> F { let norm : F = rng . sample (StandardNormal) ; norm * (self . dof / self . chi . sample (rng)) . sqrt () } }
};
}
