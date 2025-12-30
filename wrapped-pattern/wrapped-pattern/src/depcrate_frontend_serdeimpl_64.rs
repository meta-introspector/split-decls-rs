// Generated macro for impl_64 (impl)
macro_rules! Depcrate_frontend_serdeimpl_64 {
() => {
// Module: crate::frontend::serde
// Provides: {"impl_64"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de , B : PatternBackend > serde :: Deserialize < 'de > for PatternString < B > where B :: PlaceholderKeyCow < 'de > : core :: str :: FromStr , < B :: PlaceholderKeyCow < 'de > as core :: str :: FromStr > :: Err : core :: fmt :: Debug , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { let pattern_str = String :: deserialize (deserializer) ? ; let pattern = Pattern :: < B > :: try_from_str (& pattern_str , Default :: default ()) . map_err (< D :: Error as :: serde :: de :: Error > :: custom) ? ; Ok (Self (pattern)) } }
};
}
