// Generated macro for LogNormal (struct)
macro_rules! Depcrate_normalLogNormal {
() => {
// Module: crate::normal
// Provides: {"LogNormal"}
// Dependencies: {}
# [doc = " The [log-normal distribution](https://en.wikipedia.org/wiki/Log-normal_distribution) `ln N(μ, σ²)`."] # [doc = ""] # [doc = " This is the distribution of the random variable `X = exp(Y)` where `Y` is"] # [doc = " normally distributed with mean `μ` and variance `σ²`. In other words, if"] # [doc = " `X` is log-normal distributed, then `ln(X)` is `N(μ, σ²)` distributed."] # [doc = ""] # [doc = " # Plot"] # [doc = ""] # [doc = " The following diagram shows the log-normal distribution with various values"] # [doc = " of `μ` and `σ`."] # [doc = ""] # [doc = " ![Log-normal distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/log_normal.svg)"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use rand_distr::{LogNormal, Distribution};"] # [doc = ""] # [doc = " // mean 2, standard deviation 3"] # [doc = " let log_normal = LogNormal::new(2.0, 3.0).unwrap();"] # [doc = " let v = log_normal.sample(&mut rand::rng());"] # [doc = " println!(\"{} is from an ln N(2, 9) distribution\", v)"] # [doc = " ```"] # [derive (Clone , Copy , Debug , PartialEq)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct LogNormal < F > where F : Float , StandardNormal : Distribution < F > , { norm : Normal < F > , }
};
}
