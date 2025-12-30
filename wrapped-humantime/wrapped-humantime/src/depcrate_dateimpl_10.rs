// Generated macro for impl_10 (impl)
macro_rules! Depcrate_dateimpl_10 {
() => {
// Module: crate::date
// Provides: {"impl_10"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: OutOfRange => write ! (f , "numeric component is out of range") , Error :: InvalidDigit => write ! (f , "bad character where digit is expected") , Error :: InvalidFormat => write ! (f , "timestamp format is invalid") , } } }
};
}
