// Generated macro for impl_67 (impl)
macro_rules! Depcrate_checkedimpl_67 {
() => {
// Module: crate::checked
// Provides: {"impl_67"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < T : Copy + Serialize > Serialize for Checked < T > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { Option :: < T > :: from (self . 0) . serialize (serializer) } }
};
}
