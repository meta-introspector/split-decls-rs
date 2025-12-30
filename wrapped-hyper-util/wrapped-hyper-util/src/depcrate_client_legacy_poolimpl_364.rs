// Generated macro for impl_364 (impl)
macro_rules! Depcrate_client_legacy_poolimpl_364 {
() => {
// Module: crate::client::legacy::pool
// Provides: {"impl_364"}
// Dependencies: {}
impl < T : Poolable , K : Key > fmt :: Debug for Pooled < T , K > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Pooled") . field ("key" , & self . key) . finish () } }
};
}
