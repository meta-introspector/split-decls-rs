// Generated macro for impl_112 (impl)
macro_rules! Depcrate_addressimpl_112 {
() => {
// Module: crate::address
// Provides: {"impl_112"}
// Dependencies: {}
impl < T > fmt :: Debug for SendError < T > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { SendError :: Full (_) => write ! (fmt , "SendError::Full(..)") , SendError :: Closed (_) => write ! (fmt , "SendError::Closed(..)") , } } }
};
}
