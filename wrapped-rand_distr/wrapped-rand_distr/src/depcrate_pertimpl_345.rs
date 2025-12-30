// Generated macro for impl_345 (impl)
macro_rules! Depcrate_pertimpl_345 {
() => {
// Module: crate::pert
// Provides: {"impl_345"}
// Dependencies: {}
impl < F > Pert < F > where F : Float , StandardNormal : Distribution < F > , Exp1 : Distribution < F > , Open01 : Distribution < F > , { # [doc = " Construct a PERT distribution with defined `min`, `max`"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use rand_distr::Pert;"] # [doc = " let pert_dist = Pert::new(0.0, 10.0)"] # [doc = "     .with_shape(3.5)"] # [doc = "     .with_mean(3.0)"] # [doc = "     .unwrap();"] # [doc = " # let _unused: Pert<f64> = pert_dist;"] # [doc = " ```"] # [allow (clippy :: new_ret_no_self)] # [inline] pub fn new (min : F , max : F) -> PertBuilder < F > { let shape = F :: from (4.0) . unwrap () ; PertBuilder { min , max , shape } } }
};
}
