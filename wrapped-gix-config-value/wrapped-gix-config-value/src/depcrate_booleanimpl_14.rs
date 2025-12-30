// Generated macro for impl_14 (impl)
macro_rules! Depcrate_booleanimpl_14 {
() => {
// Module: crate::boolean
// Provides: {"impl_14"}
// Dependencies: {}
# [cfg (feature = "serde")] impl serde :: Serialize for Boolean { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { serializer . serialize_bool (self . 0) } }
};
}
