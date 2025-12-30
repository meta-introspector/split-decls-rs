// Generated macro for impl_43 (impl)
macro_rules! Depcrate_ustrimpl_43 {
() => {
// Module: crate::ustr
// Provides: {"impl_43"}
// Dependencies: {}
# [doc = " This impl requires enabling the optional `serde` Cargo feature"] # [cfg (feature = "serde")] impl < 'de , 'a > serde_core :: Deserialize < 'de > for & 'a PotentialUtf8 where 'de : 'a , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde_core :: Deserializer < 'de > , { if deserializer . is_human_readable () { let s = < & str > :: deserialize (deserializer) ? ; Ok (PotentialUtf8 :: from_str (s)) } else { let bytes = < & [u8] > :: deserialize (deserializer) ? ; Ok (PotentialUtf8 :: from_bytes (bytes)) } } }
};
}
