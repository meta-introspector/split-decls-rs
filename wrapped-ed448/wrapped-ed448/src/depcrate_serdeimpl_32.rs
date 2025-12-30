// Generated macro for impl_32 (impl)
macro_rules! Depcrate_serdeimpl_32 {
() => {
// Module: crate::serde
// Provides: {"impl_32"}
// Dependencies: {}
impl Serialize for Signature { fn serialize < S : ser :: Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { use ser :: SerializeTuple ; let mut seq = serializer . serialize_tuple (Signature :: BYTE_SIZE) ? ; for byte in self . to_bytes () { seq . serialize_element (& byte) ? ; } seq . end () } }
};
}
