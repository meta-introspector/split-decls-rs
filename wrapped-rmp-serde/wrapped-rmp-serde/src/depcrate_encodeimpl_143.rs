// Generated macro for impl_143 (impl)
macro_rules! Depcrate_encodeimpl_143 {
() => {
// Module: crate::encode
// Provides: {"impl_143"}
// Dependencies: {}
impl Display for Error { # [cold] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { match * self { Self :: InvalidValueWrite (ref err) => write ! (f , "invalid value write: {err}") , Self :: UnknownLength => { f . write_str ("attempt to serialize struct, sequence or map with unknown length") } Self :: InvalidDataModel (r) => write ! (f , "serialize data model is invalid: {r}") , Self :: DepthLimitExceeded => f . write_str ("depth limit exceeded") , Self :: Syntax (ref msg) => f . write_str (msg) , } } }
};
}
