// Generated macro for UnitBall (struct)
macro_rules! Depcrate_unit_ballUnitBall {
() => {
// Module: crate::unit_ball
// Provides: {"UnitBall"}
// Dependencies: {}
# [doc = " Samples uniformly from the volume of the unit ball in three dimensions."] # [doc = ""] # [doc = " Implemented via rejection sampling."] # [doc = ""] # [doc = " For a distribution that samples only from the surface of the unit ball,"] # [doc = " see [`UnitSphere`](crate::UnitSphere)."] # [doc = ""] # [doc = " For a similar distribution in two dimensions, see [`UnitDisc`](crate::UnitDisc)."] # [doc = ""] # [doc = " # Plot"] # [doc = ""] # [doc = " The following plot shows the unit ball in three dimensions."] # [doc = " This distribution samples individual points from the entire volume"] # [doc = " of the ball."] # [doc = ""] # [doc = " ![Unit ball](https://raw.githubusercontent.com/rust-random/charts/main/charts/unit_ball.svg)"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use rand_distr::{UnitBall, Distribution};"] # [doc = ""] # [doc = " let v: [f64; 3] = UnitBall.sample(&mut rand::rng());"] # [doc = " println!(\"{:?} is from the unit ball.\", v)"] # [doc = " ```"] # [derive (Clone , Copy , Debug)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct UnitBall ;
};
}
