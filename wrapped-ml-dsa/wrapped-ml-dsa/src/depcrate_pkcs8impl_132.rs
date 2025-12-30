// Generated macro for impl_132 (impl)
macro_rules! Depcrate_pkcs8impl_132 {
() => {
// Module: crate::pkcs8
// Provides: {"impl_132"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < P : MlDsaParams > SignatureBitStringEncoding for Signature < P > { fn to_bitstring (& self) -> der :: Result < BitString > { BitString :: new (0 , self . encode () . to_vec ()) } }
};
}
