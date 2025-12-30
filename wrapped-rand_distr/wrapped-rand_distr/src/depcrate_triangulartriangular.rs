// Generated macro for Triangular (struct)
macro_rules! Depcrate_triangularTriangular {
() => {
// Module: crate::triangular
// Provides: {"Triangular"}
// Dependencies: {}
# [doc = " The [triangular distribution](https://en.wikipedia.org/wiki/Triangular_distribution) `Triangular(min, max, mode)`."] # [doc = ""] # [doc = " A continuous probability distribution parameterised by a range, and a mode"] # [doc = " (most likely value) within that range."] # [doc = ""] # [doc = " The probability density function is triangular. For a similar distribution"] # [doc = " with a smooth PDF, see the [`Pert`] distribution."] # [doc = ""] # [doc = " # Plot"] # [doc = ""] # [doc = " The following plot shows the triangular distribution with various values of"] # [doc = " `min`, `max`, and `mode`."] # [doc = ""] # [doc = " ![Triangular distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/triangular.svg)"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use rand_distr::{Triangular, Distribution};"] # [doc = ""] # [doc = " let d = Triangular::new(0., 5., 2.5).unwrap();"] # [doc = " let v = d.sample(&mut rand::rng());"] # [doc = " println!(\"{} is from a triangular distribution\", v);"] # [doc = " ```"] # [doc = ""] # [doc = " [`Pert`]: crate::Pert"] # [derive (Clone , Copy , Debug , PartialEq)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Triangular < F > where F : Float , StandardUniform : Distribution < F > , { min : F , max : F , mode : F , }
};
}
