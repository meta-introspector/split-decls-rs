// Generated macro for impl_476 (impl)
macro_rules! Depcrate_zipfimpl_476 {
() => {
// Module: crate::zipf
// Provides: {"impl_476"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { Error :: STooSmall => "s < 0 or is NaN in Zipf distribution" , Error :: NTooSmall => "n < 1 in Zipf distribution" , }) } }
};
}
