// Generated macro for impl_90 (impl)
macro_rules! Depcrateimpl_90 {
() => {
// Module: crate
// Provides: {"impl_90"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for PushError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { PushError :: Full (t) => f . debug_tuple ("Full") . field (t) . finish () , PushError :: Closed (t) => f . debug_tuple ("Closed") . field (t) . finish () , } } }
};
}
