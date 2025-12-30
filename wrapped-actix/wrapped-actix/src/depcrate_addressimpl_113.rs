// Generated macro for impl_113 (impl)
macro_rules! Depcrate_addressimpl_113 {
() => {
// Module: crate::address
// Provides: {"impl_113"}
// Dependencies: {}
impl < T > fmt :: Display for SendError < T > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { SendError :: Full (_) => write ! (fmt , "send failed because receiver is full") , SendError :: Closed (_) => write ! (fmt , "send failed because receiver is gone") , } } }
};
}
