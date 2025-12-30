// Generated macro for Tuple (trait)
macro_rules! Depcrate_stats_tupleTuple {
() => {
// Module: crate::stats::tuple
// Provides: {"Tuple"}
// Dependencies: {}
# [doc = " Any tuple: `(A, B, ..)`"] pub trait Tuple : Sized { # [doc = " A tuple of distributions associated with this tuple"] type Distributions : TupledDistributions < Item = Self > ; # [doc = " A tuple of vectors associated with this tuple"] type Builder : TupledDistributionsBuilder < Item = Self > ; }
};
}
