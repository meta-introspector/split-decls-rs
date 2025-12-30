// Generated macro for impl_205 (impl)
macro_rules! Depcrate_limbimpl_205 {
() => {
// Module: crate::limb
// Provides: {"impl_205"}
// Dependencies: {}
# [cfg (feature = "serde")] impl Serialize for Limb { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . 0 . serialize (serializer) } }
};
}
