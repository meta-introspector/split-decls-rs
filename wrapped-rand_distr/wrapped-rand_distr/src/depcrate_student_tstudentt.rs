// Generated macro for StudentT (struct)
macro_rules! Depcrate_student_tStudentT {
() => {
// Module: crate::student_t
// Provides: {"StudentT"}
// Dependencies: {}
# [doc = " The [Student t-distribution](https://en.wikipedia.org/wiki/Student%27s_t-distribution) `t(ν)`."] # [doc = ""] # [doc = " The t-distribution is a continuous probability distribution"] # [doc = " parameterized by degrees of freedom `ν` (`nu`), which"] # [doc = " arises when estimating the mean of a normally-distributed"] # [doc = " population in situations where the sample size is small and"] # [doc = " the population's standard deviation is unknown."] # [doc = " It is widely used in hypothesis testing."] # [doc = ""] # [doc = " For `ν = 1`, this is equivalent to the standard"] # [doc = " [`Cauchy`](crate::Cauchy) distribution,"] # [doc = " and as `ν` diverges to infinity, `t(ν)` converges to"] # [doc = " [`StandardNormal`](crate::StandardNormal)."] # [doc = ""] # [doc = " # Plot"] # [doc = ""] # [doc = " The plot shows the t-distribution with various degrees of freedom."] # [doc = ""] # [doc = " ![T-distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/student_t.svg)"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use rand_distr::{StudentT, Distribution};"] # [doc = ""] # [doc = " let t = StudentT::new(11.0).unwrap();"] # [doc = " let v = t.sample(&mut rand::rng());"] # [doc = " println!(\"{} is from a t(11) distribution\", v)"] # [doc = " ```"] # [derive (Clone , Copy , Debug , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct StudentT < F > where F : Float , StandardNormal : Distribution < F > , Exp1 : Distribution < F > , Open01 : Distribution < F > , { chi : ChiSquared < F > , dof : F , }
};
}
