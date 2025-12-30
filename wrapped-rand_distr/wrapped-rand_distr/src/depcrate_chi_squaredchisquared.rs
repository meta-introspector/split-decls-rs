// Generated macro for ChiSquared (struct)
macro_rules! Depcrate_chi_squaredChiSquared {
() => {
// Module: crate::chi_squared
// Provides: {"ChiSquared"}
// Dependencies: {}
# [doc = " The [chi-squared distribution](https://en.wikipedia.org/wiki/Chi-squared_distribution) `χ²(k)`."] # [doc = ""] # [doc = " The chi-squared distribution is a continuous probability"] # [doc = " distribution with parameter `k > 0` degrees of freedom."] # [doc = ""] # [doc = " For `k > 0` integral, this distribution is the sum of the squares"] # [doc = " of `k` independent standard normal random variables. For other"] # [doc = " `k`, this uses the equivalent characterisation"] # [doc = " `χ²(k) = Gamma(k/2, 2)`."] # [doc = ""] # [doc = " # Plot"] # [doc = ""] # [doc = " The plot shows the chi-squared distribution with various degrees"] # [doc = " of freedom."] # [doc = ""] # [doc = " ![Chi-squared distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/chi_squared.svg)"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use rand_distr::{ChiSquared, Distribution};"] # [doc = ""] # [doc = " let chi = ChiSquared::new(11.0).unwrap();"] # [doc = " let v = chi.sample(&mut rand::rng());"] # [doc = " println!(\"{} is from a χ²(11) distribution\", v)"] # [doc = " ```"] # [derive (Clone , Copy , Debug , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct ChiSquared < F > where F : Float , StandardNormal : Distribution < F > , Exp1 : Distribution < F > , Open01 : Distribution < F > , { repr : ChiSquaredRepr < F > , }
};
}
