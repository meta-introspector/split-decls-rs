// Generated macro for impl_60 (impl)
macro_rules! Depcrateimpl_60 {
() => {
// Module: crate
// Provides: {"impl_60"}
// Dependencies: {}
# [cfg (all (feature = "alloc" , feature = "pkcs8"))] impl SignatureBitStringEncoding for Signature { fn to_bitstring (& self) -> der :: Result < BitString > { BitString :: new (0 , self . to_vec ()) } }
};
}
