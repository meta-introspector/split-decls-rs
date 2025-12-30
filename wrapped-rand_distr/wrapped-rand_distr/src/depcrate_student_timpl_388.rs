// Generated macro for impl_388 (impl)
macro_rules! Depcrate_student_timpl_388 {
() => {
// Module: crate::student_t
// Provides: {"impl_388"}
// Dependencies: {}
impl < F > StudentT < F > where F : Float , StandardNormal : Distribution < F > , Exp1 : Distribution < F > , Open01 : Distribution < F > , { # [doc = " Create a new Student t-distribution with `ν` (nu)"] # [doc = " degrees of freedom."] pub fn new (nu : F) -> Result < StudentT < F > , ChiSquaredError > { Ok (StudentT { chi : ChiSquared :: new (nu) ? , dof : nu , }) } }
};
}
