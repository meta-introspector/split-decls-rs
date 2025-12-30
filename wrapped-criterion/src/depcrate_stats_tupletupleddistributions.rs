// Generated macro for TupledDistributions (trait)
macro_rules! Depcrate_stats_tupleTupledDistributions {
() => {
// Module: crate::stats::tuple
// Provides: {"TupledDistributions"}
// Dependencies: {}
# [doc = " A tuple of distributions: `(Distribution<A>, Distribution<B>, ..)`"] pub trait TupledDistributions : Sized { # [doc = " A tuple that can be pushed/inserted into the tupled distributions"] type Item : Tuple < Distributions = Self > ; }
};
}
