// Generated macro for Pert (struct)
macro_rules! Depcrate_pertPert {
() => {
// Module: crate::pert
// Provides: {"Pert"}
// Dependencies: {}
# [doc = " The [PERT distribution](https://en.wikipedia.org/wiki/PERT_distribution) `PERT(min, max, mode, shape)`."] # [doc = ""] # [doc = " Similar to the [`Triangular`] distribution, the PERT distribution is"] # [doc = " parameterised by a range and a mode within that range. Unlike the"] # [doc = " [`Triangular`] distribution, the probability density function of the PERT"] # [doc = " distribution is smooth, with a configurable weighting around the mode."] # [doc = ""] # [doc = " # Plot"] # [doc = ""] # [doc = " The following plot shows the PERT distribution with `min = -1`, `max = 1`,"] # [doc = " and various values of `mode` and `shape`."] # [doc = ""] # [doc = " ![PERT distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/pert.svg)"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use rand_distr::{Pert, Distribution};"] # [doc = ""] # [doc = " let d = Pert::new(0., 5.).with_mode(2.5).unwrap();"] # [doc = " let v = d.sample(&mut rand::rng());"] # [doc = " println!(\"{} is from a PERT distribution\", v);"] # [doc = " ```"] # [doc = ""] # [doc = " [`Triangular`]: crate::Triangular"] # [derive (Clone , Copy , Debug , PartialEq)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Pert < F > where F : Float , StandardNormal : Distribution < F > , Exp1 : Distribution < F > , Open01 : Distribution < F > , { min : F , range : F , beta : Beta < F > , }
};
}
