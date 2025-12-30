// Generated macro for UnitCircle (struct)
macro_rules! Depcrate_unit_circleUnitCircle {
() => {
// Module: crate::unit_circle
// Provides: {"UnitCircle"}
// Dependencies: {}
# [doc = " Samples uniformly from the circumference of the unit circle in two dimensions."] # [doc = ""] # [doc = " Implemented via a method by von Neumann[^1]."] # [doc = ""] # [doc = " For a distribution that also samples from the interior of the unit circle,"] # [doc = " see [`UnitDisc`](crate::UnitDisc)."] # [doc = ""] # [doc = " For a similar distribution in three dimensions, see [`UnitSphere`](crate::UnitSphere)."] # [doc = ""] # [doc = " # Plot"] # [doc = ""] # [doc = " The following plot shows the unit circle."] # [doc = ""] # [doc = " ![Unit circle](https://raw.githubusercontent.com/rust-random/charts/main/charts/unit_circle.svg)"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use rand_distr::{UnitCircle, Distribution};"] # [doc = ""] # [doc = " let v: [f64; 2] = UnitCircle.sample(&mut rand::rng());"] # [doc = " println!(\"{:?} is from the unit circle.\", v)"] # [doc = " ```"] # [doc = ""] # [doc = " [^1]: von Neumann, J. (1951) [*Various Techniques Used in Connection with"] # [doc = "       Random Digits.*](https://mcnp.lanl.gov/pdf_files/nbs_vonneumann.pdf)"] # [doc = "       NBS Appl. Math. Ser., No. 12. Washington, DC: U.S. Government Printing"] # [doc = "       Office, pp. 36-38."] # [derive (Clone , Copy , Debug)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct UnitCircle ;
};
}
