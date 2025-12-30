// Generated macro for Pareto (struct)
macro_rules! Depcrate_paretoPareto {
() => {
// Module: crate::pareto
// Provides: {"Pareto"}
// Dependencies: {}
# [doc = " The [Pareto distribution](https://en.wikipedia.org/wiki/Pareto_distribution) `Pareto(xₘ, α)`."] # [doc = ""] # [doc = " The Pareto distribution is a continuous probability distribution with"] # [doc = " scale parameter `xₘ` ( or `k`) and shape parameter `α`."] # [doc = ""] # [doc = " # Plot"] # [doc = ""] # [doc = " The following plot shows the Pareto distribution with various values of"] # [doc = " `xₘ` and `α`."] # [doc = " Note how the shape parameter `α` corresponds to the height of the jump"] # [doc = " in density at `x = xₘ`, and to the rate of decay in the tail."] # [doc = ""] # [doc = " ![Pareto distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/pareto.svg)"] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " use rand::prelude::*;"] # [doc = " use rand_distr::Pareto;"] # [doc = ""] # [doc = " let val: f64 = rand::rng().sample(Pareto::new(1., 2.).unwrap());"] # [doc = " println!(\"{}\", val);"] # [doc = " ```"] # [derive (Clone , Copy , Debug , PartialEq)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Pareto < F > where F : Float , OpenClosed01 : Distribution < F > , { scale : F , inv_neg_shape : F , }
};
}
