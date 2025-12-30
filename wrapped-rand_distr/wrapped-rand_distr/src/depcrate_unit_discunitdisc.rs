// Generated macro for UnitDisc (struct)
macro_rules! Depcrate_unit_discUnitDisc {
() => {
// Module: crate::unit_disc
// Provides: {"UnitDisc"}
// Dependencies: {}
# [doc = " Samples uniformly from the unit disc in two dimensions."] # [doc = ""] # [doc = " Implemented via rejection sampling."] # [doc = ""] # [doc = " For a distribution that samples only from the circumference of the unit disc,"] # [doc = " see [`UnitCircle`](crate::UnitCircle)."] # [doc = ""] # [doc = " For a similar distribution in three dimensions, see [`UnitBall`](crate::UnitBall)."] # [doc = ""] # [doc = " # Plot"] # [doc = ""] # [doc = " The following plot shows the unit disc."] # [doc = " This distribution samples individual points from the entire area of the disc."] # [doc = ""] # [doc = " ![Unit disc](https://raw.githubusercontent.com/rust-random/charts/main/charts/unit_disc.svg)"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use rand_distr::{UnitDisc, Distribution};"] # [doc = ""] # [doc = " let v: [f64; 2] = UnitDisc.sample(&mut rand::rng());"] # [doc = " println!(\"{:?} is from the unit Disc.\", v)"] # [doc = " ```"] # [derive (Clone , Copy , Debug)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct UnitDisc ;
};
}
