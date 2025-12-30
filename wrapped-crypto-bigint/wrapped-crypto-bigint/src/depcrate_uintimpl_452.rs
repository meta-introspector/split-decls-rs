// Generated macro for impl_452 (impl)
macro_rules! Depcrate_uintimpl_452 {
() => {
// Module: crate::uint
// Provides: {"impl_452"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de , const LIMBS : usize > Deserialize < 'de > for Uint < LIMBS > where Uint < LIMBS > : Encoding , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let mut buffer = Self :: ZERO . to_le_bytes () ; serdect :: array :: deserialize_hex_or_bin (buffer . as_mut () , deserializer) ? ; Ok (Self :: from_le_bytes (buffer)) } }
};
}
