// Generated macro for WnafGroup (trait)
macro_rules! Depcrate_wnafWnafGroup {
() => {
// Module: crate::wnaf
// Provides: {"WnafGroup"}
// Dependencies: {}
# [doc = " Extension trait on a [`Group`] that provides helpers used by [`Wnaf`]."] pub trait WnafGroup : Group { # [doc = " Recommends a wNAF window size given the number of scalars you intend to multiply"] # [doc = " a base by. Always returns a number between 2 and 22, inclusive."] fn recommended_wnaf_for_num_scalars (num_scalars : usize) -> usize ; }
};
}
