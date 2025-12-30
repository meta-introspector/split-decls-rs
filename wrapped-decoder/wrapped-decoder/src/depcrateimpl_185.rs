// Generated macro for impl_185 (impl)
macro_rules! Depcrateimpl_185 {
() => {
// Module: crate
// Provides: {"impl_185"}
// Dependencies: {}
impl fmt :: Display for DecodeError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { DecodeError :: UnexpectedEof => f . write_str ("unexpected end of stream") , DecodeError :: Malformed => f . write_str ("malformed data") , } } }
};
}
