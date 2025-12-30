// Generated macro for impl_894 (impl)
macro_rules! Depcrate_timestampimpl_894 {
() => {
// Module: crate::timestamp
// Provides: {"impl_894"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de > serde_core :: Deserialize < 'de > for Timestamp { # [inline] fn deserialize < D : serde_core :: Deserializer < 'de > > (deserializer : D ,) -> Result < Timestamp , D :: Error > { use serde_core :: de ; struct TimestampVisitor ; impl < 'de > de :: Visitor < 'de > for TimestampVisitor { type Value = Timestamp ; fn expecting (& self , f : & mut core :: fmt :: Formatter ,) -> core :: fmt :: Result { f . write_str ("a timestamp string") } # [inline] fn visit_bytes < E : de :: Error > (self , value : & [u8] ,) -> Result < Timestamp , E > { DEFAULT_DATETIME_PARSER . parse_timestamp (value) . map_err (de :: Error :: custom) } # [inline] fn visit_str < E : de :: Error > (self , value : & str ,) -> Result < Timestamp , E > { self . visit_bytes (value . as_bytes ()) } } deserializer . deserialize_str (TimestampVisitor) } }
};
}
