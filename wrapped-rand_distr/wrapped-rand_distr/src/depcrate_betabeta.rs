// Generated macro for Beta (struct)
macro_rules! Depcrate_betaBeta {
() => {
// Module: crate::beta
// Provides: {"Beta"}
// Dependencies: {}
# [doc = " The [Beta distribution](https://en.wikipedia.org/wiki/Beta_distribution) `Beta(α, β)`."] # [doc = ""] # [doc = " The Beta distribution is a continuous probability distribution"] # [doc = " defined on the interval `[0, 1]`. It is the conjugate prior for the"] # [doc = " parameter `p` of the [`Binomial`][crate::Binomial] distribution."] # [doc = ""] # [doc = " It has two shape parameters `α` (alpha) and `β` (beta) which control"] # [doc = " the shape of the distribution. Both `a` and `β` must be greater than zero."] # [doc = " The distribution is symmetric when `α = β`."] # [doc = ""] # [doc = " # Plot"] # [doc = ""] # [doc = " The plot shows the Beta distribution with various combinations"] # [doc = " of `α` and `β`."] # [doc = ""] # [doc = " ![Beta distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/beta.svg)"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use rand_distr::{Distribution, Beta};"] # [doc = ""] # [doc = " let beta = Beta::new(2.0, 5.0).unwrap();"] # [doc = " let v = beta.sample(&mut rand::rng());"] # [doc = " println!(\"{} is from a Beta(2, 5) distribution\", v);"] # [doc = " ```"] # [derive (Clone , Copy , Debug , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Beta < F > where F : Float , Open01 : Distribution < F > , { a : F , b : F , switched_params : bool , algorithm : BetaAlgorithm < F > , }
};
}
