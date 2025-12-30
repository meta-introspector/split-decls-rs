// Generated macro for impl_52 (impl)
macro_rules! Depcrate_civil_dateimpl_52 {
() => {
// Module: crate::civil::date
// Provides: {"impl_52"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de > serde_core :: Deserialize < 'de > for Date { # [inline] fn deserialize < D : serde_core :: Deserializer < 'de > > (deserializer : D ,) -> Result < Date , D :: Error > { use serde_core :: de ; struct DateVisitor ; impl < 'de > de :: Visitor < 'de > for DateVisitor { type Value = Date ; fn expecting (& self , f : & mut core :: fmt :: Formatter ,) -> core :: fmt :: Result { f . write_str ("a date string") } # [inline] fn visit_bytes < E : de :: Error > (self , value : & [u8] ,) -> Result < Date , E > { DEFAULT_DATETIME_PARSER . parse_date (value) . map_err (de :: Error :: custom) } # [inline] fn visit_str < E : de :: Error > (self , value : & str) -> Result < Date , E > { self . visit_bytes (value . as_bytes ()) } } deserializer . deserialize_str (DateVisitor) } }
};
}
