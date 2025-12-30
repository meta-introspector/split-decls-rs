// Generated macro for impl_510 (impl)
macro_rules! Depcrate_wrappingimpl_510 {
() => {
// Module: crate::wrapping
// Provides: {"impl_510"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < T : Serialize > Serialize for Wrapping < T > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . 0 . serialize (serializer) } }
};
}
