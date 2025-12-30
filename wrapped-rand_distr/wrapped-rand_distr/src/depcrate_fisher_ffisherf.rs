// Generated macro for FisherF (struct)
macro_rules! Depcrate_fisher_fFisherF {
() => {
// Module: crate::fisher_f
// Provides: {"FisherF"}
// Dependencies: {}
# [doc = " The [Fisher F-distribution](https://en.wikipedia.org/wiki/F-distribution) `F(m, n)`."] # [doc = ""] # [doc = " This distribution is equivalent to the ratio of two normalised"] # [doc = " chi-squared distributions, that is, `F(m,n) = (χ²(m)/m) /"] # [doc = " (χ²(n)/n)`."] # [doc = ""] # [doc = " # Plot"] # [doc = ""] # [doc = " The plot shows the F-distribution with various values of `m` and `n`."] # [doc = ""] # [doc = " ![F-distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/fisher_f.svg)"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use rand_distr::{FisherF, Distribution};"] # [doc = ""] # [doc = " let f = FisherF::new(2.0, 32.0).unwrap();"] # [doc = " let v = f.sample(&mut rand::rng());"] # [doc = " println!(\"{} is from an F(2, 32) distribution\", v)"] # [doc = " ```"] # [derive (Clone , Copy , Debug , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct FisherF < F > where F : Float , StandardNormal : Distribution < F > , Exp1 : Distribution < F > , Open01 : Distribution < F > , { numer : ChiSquared < F > , denom : ChiSquared < F > , dof_ratio : F , }
};
}
