// Generated macro for impl_1075 (impl)
macro_rules! Depcrate_base_unitimpl_1075 {
() => {
// Module: crate::base::unit
// Provides: {"impl_1075"}
// Dependencies: {}
# [cfg (feature = "serde-serialize-no-std")] impl < T : Serialize > Serialize for Unit < T > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . value . serialize (serializer) } }
};
}
