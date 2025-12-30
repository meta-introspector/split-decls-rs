// Generated macro for StandardGeometric (struct)
macro_rules! Depcrate_geometricStandardGeometric {
() => {
// Module: crate::geometric
// Provides: {"StandardGeometric"}
// Dependencies: {}
# [doc = " The standard geometric distribution `Geometric(0.5)`."] # [doc = ""] # [doc = " This is equivalent to `Geometric::new(0.5)`, but faster."] # [doc = ""] # [doc = " See [`Geometric`](crate::Geometric) for the general geometric distribution."] # [doc = ""] # [doc = " # Plot"] # [doc = ""] # [doc = " The following plot illustrates the standard geometric distribution."] # [doc = ""] # [doc = " ![Standard Geometric distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/standard_geometric.svg)"] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " use rand::prelude::*;"] # [doc = " use rand_distr::StandardGeometric;"] # [doc = ""] # [doc = " let v = StandardGeometric.sample(&mut rand::rng());"] # [doc = " println!(\"{} is from a Geometric(0.5) distribution\", v);"] # [doc = " ```"] # [doc = ""] # [doc = " # Notes"] # [doc = " Implemented via iterated"] # [doc = " [`Rng::gen::<u64>().leading_zeros()`](Rng::gen::<u64>().leading_zeros())."] # [derive (Copy , Clone , Debug)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct StandardGeometric ;
};
}
