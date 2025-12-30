// Generated macro for impl_407 (impl)
macro_rules! Depcrate_pkcs8impl_407 {
() => {
// Module: crate::pkcs8
// Provides: {"impl_407"}
// Dependencies: {}
impl Template { # [inline] fn alg_id_value (& self) -> untrusted :: Input < '_ > { untrusted :: Input :: from (self . alg_id_value_ ()) } fn alg_id_value_ (& self) -> & [u8] { & self . bytes [self . alg_id_range . start .. self . alg_id_range . end] } # [inline] pub fn curve_oid (& self) -> untrusted :: Input < '_ > { untrusted :: Input :: from (& self . alg_id_value_ () [self . curve_id_index ..]) } }
};
}
