// Generated macro for UnitSphere (struct)
macro_rules! Depcrate_unit_sphereUnitSphere {
() => {
// Module: crate::unit_sphere
// Provides: {"UnitSphere"}
// Dependencies: {}
# [doc = " Samples uniformly from the surface of the unit sphere in three dimensions."] # [doc = ""] # [doc = " Implemented via a method by Marsaglia[^1]."] # [doc = ""] # [doc = " For a distribution that also samples from the interior of the sphere,"] # [doc = " see [`UnitBall`](crate::UnitBall)."] # [doc = ""] # [doc = " For a similar distribution in two dimensions, see [`UnitCircle`](crate::UnitCircle)."] # [doc = ""] # [doc = " # Plot"] # [doc = ""] # [doc = " The following plot shows the unit sphere as a wireframe."] # [doc = " The wireframe is meant to illustrate that this distribution samples"] # [doc = " from the surface of the sphere only, not from the interior."] # [doc = ""] # [doc = " ![Unit sphere](https://raw.githubusercontent.com/rust-random/charts/main/charts/unit_sphere.svg)"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use rand_distr::{UnitSphere, Distribution};"] # [doc = ""] # [doc = " let v: [f64; 3] = UnitSphere.sample(&mut rand::rng());"] # [doc = " println!(\"{:?} is from the unit sphere surface.\", v)"] # [doc = " ```"] # [doc = ""] # [doc = " [^1]: Marsaglia, George (1972). [*Choosing a Point from the Surface of a"] # [doc = "       Sphere.*](https://doi.org/10.1214/aoms/1177692644)"] # [doc = "       Ann. Math. Statist. 43, no. 2, 645--646."] # [derive (Clone , Copy , Debug)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct UnitSphere ;
};
}
