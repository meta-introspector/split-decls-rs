// Generated macro for impl_300 (impl)
macro_rules! Depcrate_hmacimpl_300 {
() => {
// Module: crate::hmac
// Provides: {"impl_300"}
// Dependencies: {}
impl core :: fmt :: Debug for Context { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> Result < () , core :: fmt :: Error > { f . debug_struct ("Context") . field ("algorithm" , & self . key . algorithm . digest_algorithm ()) . finish () } }
};
}
