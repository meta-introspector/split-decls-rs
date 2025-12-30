// Generated macro for BernoulliError (enum)
macro_rules! Depcrate_distr_bernoulliBernoulliError {
() => {
// Module: crate::distr::bernoulli
// Provides: {"BernoulliError"}
// Dependencies: {}
# [doc = " Error type returned from [`Bernoulli::new`]."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum BernoulliError { # [doc = " `p < 0` or `p > 1`."] InvalidProbability , }
};
}
