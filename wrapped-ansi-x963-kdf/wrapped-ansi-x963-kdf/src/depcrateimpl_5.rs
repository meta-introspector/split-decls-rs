// Generated macro for impl_5 (impl)
macro_rules! Depcrateimpl_5 {
() => {
// Module: crate
// Provides: {"impl_5"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { f . write_str (match self { Error :: NoSecret => "Buffer for secret has zero length." , Error :: NoOutput => "Buffer for key has zero length." , Error :: InputOverflow => "Input length is to big." , Error :: CounterOverflow => "Requested key length is to big." , }) } }
};
}
