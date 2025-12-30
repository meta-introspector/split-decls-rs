// Generated macro for impl_193 (impl)
macro_rules! Depcrate_civil_timeimpl_193 {
() => {
// Module: crate::civil::time
// Provides: {"impl_193"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de > serde_core :: Deserialize < 'de > for Time { # [inline] fn deserialize < D : serde_core :: Deserializer < 'de > > (deserializer : D ,) -> Result < Time , D :: Error > { use serde_core :: de ; struct TimeVisitor ; impl < 'de > de :: Visitor < 'de > for TimeVisitor { type Value = Time ; fn expecting (& self , f : & mut core :: fmt :: Formatter ,) -> core :: fmt :: Result { f . write_str ("a time string") } # [inline] fn visit_bytes < E : de :: Error > (self , value : & [u8] ,) -> Result < Time , E > { DEFAULT_DATETIME_PARSER . parse_time (value) . map_err (de :: Error :: custom) } # [inline] fn visit_str < E : de :: Error > (self , value : & str) -> Result < Time , E > { self . visit_bytes (value . as_bytes ()) } } deserializer . deserialize_str (TimeVisitor) } }
};
}
