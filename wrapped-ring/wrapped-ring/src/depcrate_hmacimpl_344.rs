// Generated macro for impl_344 (impl)
macro_rules! Depcrate_hmacimpl_344 {
() => {
// Module: crate::hmac
// Provides: {"impl_344"}
// Dependencies: {}
impl core :: fmt :: Debug for Key { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> Result < () , core :: fmt :: Error > { f . debug_struct ("Key") . field ("algorithm" , self . algorithm () . digest_algorithm ()) . finish () } }
};
}
