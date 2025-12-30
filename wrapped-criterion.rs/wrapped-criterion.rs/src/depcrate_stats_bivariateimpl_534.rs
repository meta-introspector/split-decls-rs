// Generated macro for impl_534 (impl)
macro_rules! Depcrate_stats_bivariateimpl_534 {
() => {
// Module: crate::stats::bivariate
// Provides: {"impl_534"}
// Dependencies: {}
impl < 'a , X , Y > Data < 'a , X , Y > { # [doc = " Returns the length of the data set"] pub fn len (& self) -> usize { self . 0 . len () } # [doc = " Iterate over the data set"] pub fn iter (& self) -> Pairs < 'a , X , Y > { Pairs { data : * self , state : 0 , } } }
};
}
