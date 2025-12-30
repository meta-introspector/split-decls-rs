// Generated macro for Error (enum)
macro_rules! Depcrate_distr_weightedError {
() => {
// Module: crate::distr::weighted
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Invalid weight errors"] # [doc = ""] # [doc = " This type represents errors from [`WeightedIndex::new`],"] # [doc = " [`WeightedIndex::update_weights`] and other weighted distributions."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] # [non_exhaustive] pub enum Error { # [doc = " The input weight sequence is empty, too long, or wrongly ordered"] InvalidInput , # [doc = " A weight is negative, too large for the distribution, or not a valid number"] InvalidWeight , # [doc = " Not enough non-zero weights are available to sample values"] # [doc = ""] # [doc = " When attempting to sample a single value this implies that all weights"] # [doc = " are zero. When attempting to sample `amount` values this implies that"] # [doc = " less than `amount` weights are greater than zero."] InsufficientNonZero , # [doc = " Overflow when calculating the sum of weights"] Overflow , }
};
}
