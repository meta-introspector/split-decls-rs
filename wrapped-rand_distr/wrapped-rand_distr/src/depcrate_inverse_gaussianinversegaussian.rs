// Generated macro for InverseGaussian (struct)
macro_rules! Depcrate_inverse_gaussianInverseGaussian {
() => {
// Module: crate::inverse_gaussian
// Provides: {"InverseGaussian"}
// Dependencies: {}
# [doc = " The [inverse Gaussian distribution](https://en.wikipedia.org/wiki/Inverse_Gaussian_distribution) `IG(μ, λ)`."] # [doc = ""] # [doc = " This is a continuous probability distribution with mean parameter `μ` (`mu`)"] # [doc = " and shape parameter `λ` (`lambda`), defined for `x > 0`."] # [doc = " It is also known as the Wald distribution."] # [doc = ""] # [doc = " # Plot"] # [doc = ""] # [doc = " The following plot shows the inverse Gaussian distribution"] # [doc = " with various values of `μ` and `λ`."] # [doc = ""] # [doc = " ![Inverse Gaussian distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/inverse_gaussian.svg)"] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " use rand_distr::{InverseGaussian, Distribution};"] # [doc = ""] # [doc = " let inv_gauss = InverseGaussian::new(1.0, 2.0).unwrap();"] # [doc = " let v = inv_gauss.sample(&mut rand::rng());"] # [doc = " println!(\"{} is from a inverse Gaussian(1, 2) distribution\", v);"] # [doc = " ```"] # [derive (Debug , Clone , Copy , PartialEq)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct InverseGaussian < F > where F : Float , StandardNormal : Distribution < F > , StandardUniform : Distribution < F > , { mean : F , shape : F , }
};
}
