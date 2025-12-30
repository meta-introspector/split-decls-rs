// Generated macro for Gumbel (struct)
macro_rules! Depcrate_gumbelGumbel {
() => {
// Module: crate::gumbel
// Provides: {"Gumbel"}
// Dependencies: {}
# [doc = " The [Gumbel distribution](https://en.wikipedia.org/wiki/Gumbel_distribution) `Gumbel(μ, β)`."] # [doc = ""] # [doc = " The Gumbel distribution is a continuous probability distribution"] # [doc = " with location parameter `μ` (`mu`) and scale parameter `β` (`beta`)."] # [doc = " It is used to model the distribution of the maximum (or minimum)"] # [doc = " of a number of samples of various distributions."] # [doc = ""] # [doc = " # Density function"] # [doc = ""] # [doc = " `f(x) = exp(-(z + exp(-z))) / β`, where `z = (x - μ) / β`."] # [doc = ""] # [doc = " # Plot"] # [doc = ""] # [doc = " The following plot illustrates the Gumbel distribution with various values of `μ` and `β`."] # [doc = " Note how the location parameter `μ` shifts the distribution along the x-axis,"] # [doc = " and the scale parameter `β` changes the density around `μ`."] # [doc = " Note also the asymptotic behavior of the distribution towards the right."] # [doc = ""] # [doc = " ![Gumbel distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/gumbel.svg)"] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " use rand::prelude::*;"] # [doc = " use rand_distr::Gumbel;"] # [doc = ""] # [doc = " let val: f64 = rand::rng().sample(Gumbel::new(0.0, 1.0).unwrap());"] # [doc = " println!(\"{}\", val);"] # [doc = " ```"] # [derive (Clone , Copy , Debug , PartialEq)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Gumbel < F > where F : Float , OpenClosed01 : Distribution < F > , { location : F , scale : F , }
};
}
