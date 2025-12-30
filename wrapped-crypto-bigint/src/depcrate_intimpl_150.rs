// Generated macro for impl_150 (impl)
macro_rules! Depcrate_intimpl_150 {
() => {
// Module: crate::int
// Provides: {"impl_150"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < const LIMBS : usize > Serialize for Int < LIMBS > where Int < LIMBS > : Encoding , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serdect :: array :: serialize_hex_lower_or_bin (& Encoding :: to_le_bytes (self) , serializer) } }
};
}
