// Generated macro for impl_41 (impl)
macro_rules! Depcrate_integerimpl_41 {
() => {
// Module: crate::integer
// Provides: {"impl_41"}
// Dependencies: {}
# [cfg (feature = "serde")] impl serde :: Serialize for Integer { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { if let Some (suffix) = self . suffix { serializer . serialize_i64 (self . value << suffix . bitwise_offset ()) } else { serializer . serialize_i64 (self . value) } } }
};
}
