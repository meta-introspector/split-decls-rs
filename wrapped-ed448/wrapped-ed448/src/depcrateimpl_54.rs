// Generated macro for impl_54 (impl)
macro_rules! Depcrateimpl_54 {
() => {
// Module: crate
// Provides: {"impl_54"}
// Dependencies: {}
# [cfg (all (feature = "alloc" , feature = "pkcs8"))] impl SignatureBitStringEncoding for Signature { fn to_bitstring (& self) -> der :: Result < BitString > { BitString :: new (0 , self . to_bytes ()) } }
};
}
