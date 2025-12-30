// Generated macro for impl_20 (impl)
macro_rules! Depcrate_distr_bernoulliimpl_20 {
() => {
// Module: crate::distr::bernoulli
// Provides: {"impl_20"}
// Dependencies: {}
impl fmt :: Display for BernoulliError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { BernoulliError :: InvalidProbability => "p is outside [0, 1] in Bernoulli distribution" , }) } }
};
}
