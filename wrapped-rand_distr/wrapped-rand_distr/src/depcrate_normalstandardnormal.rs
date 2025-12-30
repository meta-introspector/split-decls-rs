// Generated macro for StandardNormal (struct)
macro_rules! Depcrate_normalStandardNormal {
() => {
// Module: crate::normal
// Provides: {"StandardNormal"}
// Dependencies: {}
# [doc = " The standard Normal distribution `N(0, 1)`."] # [doc = ""] # [doc = " This is equivalent to `Normal::new(0.0, 1.0)`, but faster."] # [doc = ""] # [doc = " See [`Normal`](crate::Normal) for the general Normal distribution."] # [doc = ""] # [doc = " # Plot"] # [doc = ""] # [doc = " The following diagram shows the standard Normal distribution."] # [doc = ""] # [doc = " ![Standard Normal distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/standard_normal.svg)"] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " use rand::prelude::*;"] # [doc = " use rand_distr::StandardNormal;"] # [doc = ""] # [doc = " let val: f64 = rand::rng().sample(StandardNormal);"] # [doc = " println!(\"{}\", val);"] # [doc = " ```"] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " Implemented via the ZIGNOR variant[^1] of the Ziggurat method."] # [doc = ""] # [doc = " [^1]: Jurgen A. Doornik (2005). [*An Improved Ziggurat Method to"] # [doc = "       Generate Normal Random Samples*]("] # [doc = "       https://www.doornik.com/research/ziggurat.pdf)."] # [doc = "       Nuffield College, Oxford"] # [derive (Clone , Copy , Debug)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct StandardNormal ;
};
}
