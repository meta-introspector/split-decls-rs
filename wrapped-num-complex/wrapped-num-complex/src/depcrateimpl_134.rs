// Generated macro for impl_134 (impl)
macro_rules! Depcrateimpl_134 {
() => {
// Module: crate
// Provides: {"impl_134"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < T > serde :: Serialize for Complex < T > where T : serde :: Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { (& self . re , & self . im) . serialize (serializer) } }
};
}
