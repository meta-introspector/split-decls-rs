// Generated macro for impl_644 (impl)
macro_rules! Depcrate_utilsimpl_644 {
() => {
// Module: crate::utils
// Provides: {"impl_644"}
// Dependencies: {}
# [cfg (feature = "serialize")] impl Serialize for ByteBuf { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_bytes (& self . 0) } }
};
}
