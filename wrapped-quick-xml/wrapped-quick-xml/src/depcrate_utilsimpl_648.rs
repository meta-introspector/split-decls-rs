// Generated macro for impl_648 (impl)
macro_rules! Depcrate_utilsimpl_648 {
() => {
// Module: crate::utils
// Provides: {"impl_648"}
// Dependencies: {}
# [cfg (feature = "serialize")] impl < 'de > Serialize for Bytes < 'de > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_bytes (self . 0) } }
};
}
