// Generated macro for impl_461 (impl)
macro_rules! Depcrate_uintimpl_461 {
() => {
// Module: crate::uint
// Provides: {"impl_461"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < const LIMBS : usize > Serialize for Uint < LIMBS > where Uint < LIMBS > : Encoding , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { serdect :: array :: serialize_hex_lower_or_bin (& Encoding :: to_le_bytes (self) , serializer) } }
};
}
