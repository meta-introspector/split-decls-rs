// Generated macro for impl_225 (impl)
macro_rules! Depcrate_distr_uniformimpl_225 {
() => {
// Module: crate::distr::uniform
// Provides: {"impl_225"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { Error :: EmptyRange => "low > high (or equal if exclusive) in uniform distribution" , Error :: NonFinite => "Non-finite range in uniform distribution" , }) } }
};
}
