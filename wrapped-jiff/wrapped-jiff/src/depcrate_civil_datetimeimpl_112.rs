// Generated macro for impl_112 (impl)
macro_rules! Depcrate_civil_datetimeimpl_112 {
() => {
// Module: crate::civil::datetime
// Provides: {"impl_112"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de > serde_core :: Deserialize < 'de > for DateTime { # [inline] fn deserialize < D : serde_core :: Deserializer < 'de > > (deserializer : D ,) -> Result < DateTime , D :: Error > { use serde_core :: de ; struct DateTimeVisitor ; impl < 'de > de :: Visitor < 'de > for DateTimeVisitor { type Value = DateTime ; fn expecting (& self , f : & mut core :: fmt :: Formatter ,) -> core :: fmt :: Result { f . write_str ("a datetime string") } # [inline] fn visit_bytes < E : de :: Error > (self , value : & [u8] ,) -> Result < DateTime , E > { DEFAULT_DATETIME_PARSER . parse_datetime (value) . map_err (de :: Error :: custom) } # [inline] fn visit_str < E : de :: Error > (self , value : & str ,) -> Result < DateTime , E > { self . visit_bytes (value . as_bytes ()) } } deserializer . deserialize_str (DateTimeVisitor) } }
};
}
