// Generated macro for impl_45 (impl)
macro_rules! Depcrate_serdeimpl_45 {
() => {
// Module: crate::serde
// Provides: {"impl_45"}
// Dependencies: {}
# [cfg (feature = "serde_bytes")] impl serde_bytes :: Serialize for Signature { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serializer . serialize_bytes (& self . to_bytes ()) } }
};
}
