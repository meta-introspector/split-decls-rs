// Generated macro for RemMixed (trait)
macro_rules! Depcrate_traitsRemMixed {
() => {
// Module: crate::traits
// Provides: {"RemMixed"}
// Dependencies: {}
# [doc = " Support for calculating the remainder of two differently sized integers."] pub trait RemMixed < Reductor > : Sized { # [doc = " Calculate the remainder of `self` by the `reductor`."] fn rem_mixed (& self , reductor : & NonZero < Reductor >) -> Reductor ; }
};
}
