// Generated macro for impl_96 (impl)
macro_rules! Depcrate_kv_valueimpl_96 {
() => {
// Module: crate::kv::value
// Provides: {"impl_96"}
// Dependencies: {}
# [cfg (feature = "kv_serde")] impl < 'v > serde :: Serialize for Value < 'v > { fn serialize < S > (& self , s : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { self . inner . serialize (s) } }
};
}
