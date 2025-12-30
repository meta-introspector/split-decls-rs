// Generated macro for impl_232 (impl)
macro_rules! Depcrate_distr_weightedimpl_232 {
() => {
// Module: crate::distr::weighted
// Provides: {"impl_232"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str (match * self { Error :: InvalidInput => "Weights sequence is empty/too long/unordered" , Error :: InvalidWeight => "A weight is negative, too large or not a valid number" , Error :: InsufficientNonZero => "Not enough weights > zero" , Error :: Overflow => "Overflow when summing weights" , }) } }
};
}
