// Generated macro for impl_42 (impl)
macro_rules! Depcrate_ustrimpl_42 {
() => {
// Module: crate::ustr
// Provides: {"impl_42"}
// Dependencies: {}
# [doc = " This impl requires enabling the optional `serde` Cargo feature"] # [cfg (all (feature = "serde" , feature = "alloc"))] impl < 'de > serde_core :: Deserialize < 'de > for Box < PotentialUtf8 > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde_core :: Deserializer < 'de > , { if deserializer . is_human_readable () { let boxed_str = Box :: < str > :: deserialize (deserializer) ? ; Ok (PotentialUtf8 :: from_boxed_str (boxed_str)) } else { let boxed_bytes = Box :: < [u8] > :: deserialize (deserializer) ? ; Ok (PotentialUtf8 :: from_boxed_bytes (boxed_bytes)) } } }
};
}
