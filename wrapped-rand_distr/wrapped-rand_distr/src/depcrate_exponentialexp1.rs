// Generated macro for Exp1 (struct)
macro_rules! Depcrate_exponentialExp1 {
() => {
// Module: crate::exponential
// Provides: {"Exp1"}
// Dependencies: {}
# [doc = " The standard exponential distribution `Exp(1)`."] # [doc = ""] # [doc = " This is equivalent to `Exp::new(1.0)` or sampling with"] # [doc = " `-rng.gen::<f64>().ln()`, but faster."] # [doc = ""] # [doc = " See [`Exp`](crate::Exp) for the general exponential distribution."] # [doc = ""] # [doc = " # Plot"] # [doc = ""] # [doc = " The following plot illustrates the exponential distribution with `λ = 1`."] # [doc = ""] # [doc = " ![Exponential distribution](https://raw.githubusercontent.com/rust-random/charts/main/charts/exponential_exp1.svg)"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use rand::prelude::*;"] # [doc = " use rand_distr::Exp1;"] # [doc = ""] # [doc = " let val: f64 = rand::rng().sample(Exp1);"] # [doc = " println!(\"{}\", val);"] # [doc = " ```"] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " Implemented via the ZIGNOR variant[^1] of the Ziggurat method. The exact"] # [doc = " description in the paper was adjusted to use tables for the exponential"] # [doc = " distribution rather than normal."] # [doc = ""] # [doc = " [^1]: Jurgen A. Doornik (2005). [*An Improved Ziggurat Method to"] # [doc = "       Generate Normal Random Samples*]("] # [doc = "       https://www.doornik.com/research/ziggurat.pdf)."] # [doc = "       Nuffield College, Oxford"] # [derive (Clone , Copy , Debug)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Exp1 ;
};
}
