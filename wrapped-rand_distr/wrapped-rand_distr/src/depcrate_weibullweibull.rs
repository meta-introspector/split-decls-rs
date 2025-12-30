// Generated macro for Weibull (struct)
macro_rules! Depcrate_weibullWeibull {
() => {
// Module: crate::weibull
// Provides: {"Weibull"}
// Dependencies: {}
# [doc = " The [Weibull distribution](https://en.wikipedia.org/wiki/Weibull_distribution) `Weibull(λ, k)`."] # [doc = ""] # [doc = " This is a family of continuous probability distributions with"] # [doc = " scale parameter `λ` (`lambda`) and shape parameter `k`. It is used"] # [doc = " to model reliability data, life data, and accelerated life testing data."] # [doc = ""] # [doc = " # Density function"] # [doc = ""] # [doc = " `f(x; λ, k) = (k / λ) * (x / λ)^(k - 1) * exp(-(x / λ)^k)` for `x >= 0`."] # [doc = ""] # [doc = " # Plot"] # [doc = ""] # [doc = " The following plot shows the Weibull distribution with various values of `λ` and `k`."] # [doc = ""] # [doc = " ![Weibull distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/weibull.svg)"] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " use rand::prelude::*;"] # [doc = " use rand_distr::Weibull;"] # [doc = ""] # [doc = " let val: f64 = rand::rng().sample(Weibull::new(1., 10.).unwrap());"] # [doc = " println!(\"{}\", val);"] # [doc = " ```"] # [doc = ""] # [doc = " # Numerics"] # [doc = ""] # [doc = " For small `k` like `< 0.005`, even with `f64` a significant number of samples will be so small that they underflow to `0.0`"] # [doc = " or so big they overflow to `inf`. This is a limitation of the floating point representation and not specific to this implementation."] # [derive (Clone , Copy , Debug , PartialEq)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Weibull < F > where F : Float , OpenClosed01 : Distribution < F > , { inv_shape : F , scale : F , }
};
}
