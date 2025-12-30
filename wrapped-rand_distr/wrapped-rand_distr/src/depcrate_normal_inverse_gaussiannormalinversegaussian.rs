// Generated macro for NormalInverseGaussian (struct)
macro_rules! Depcrate_normal_inverse_gaussianNormalInverseGaussian {
() => {
// Module: crate::normal_inverse_gaussian
// Provides: {"NormalInverseGaussian"}
// Dependencies: {}
# [doc = " The [normal-inverse Gaussian distribution](https://en.wikipedia.org/wiki/Normal-inverse_Gaussian_distribution) `NIG(α, β)`."] # [doc = ""] # [doc = " This is a continuous probability distribution with two parameters,"] # [doc = " `α` (`alpha`) and `β` (`beta`), defined in `(-∞, ∞)`."] # [doc = " It is also known as the normal-Wald distribution."] # [doc = ""] # [doc = " # Plot"] # [doc = ""] # [doc = " The following plot shows the normal-inverse Gaussian distribution with various values of `α` and `β`."] # [doc = ""] # [doc = " ![Normal-inverse Gaussian distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/normal_inverse_gaussian.svg)"] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " use rand_distr::{NormalInverseGaussian, Distribution};"] # [doc = ""] # [doc = " let norm_inv_gauss = NormalInverseGaussian::new(2.0, 1.0).unwrap();"] # [doc = " let v = norm_inv_gauss.sample(&mut rand::rng());"] # [doc = " println!(\"{} is from a normal-inverse Gaussian(2, 1) distribution\", v);"] # [doc = " ```"] # [derive (Debug , Clone , Copy , PartialEq)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct NormalInverseGaussian < F > where F : Float , StandardNormal : Distribution < F > , StandardUniform : Distribution < F > , { beta : F , inverse_gaussian : InverseGaussian < F > , }
};
}
