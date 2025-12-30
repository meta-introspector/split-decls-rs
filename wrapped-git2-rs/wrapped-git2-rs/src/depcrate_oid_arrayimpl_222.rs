// Generated macro for impl_222 (impl)
macro_rules! Depcrate_oid_arrayimpl_222 {
() => {
// Module: crate::oid_array
// Provides: {"impl_222"}
// Dependencies: {}
impl < 'repo > std :: fmt :: Debug for OidArray { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { f . debug_tuple ("OidArray") . field (& self . deref ()) . finish () } }
};
}
