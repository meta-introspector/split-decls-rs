// Generated macro for impl_161 (impl)
macro_rules! Depcrate_civil_iso_week_dateimpl_161 {
() => {
// Module: crate::civil::iso_week_date
// Provides: {"impl_161"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de > serde_core :: Deserialize < 'de > for ISOWeekDate { # [inline] fn deserialize < D : serde_core :: Deserializer < 'de > > (deserializer : D ,) -> Result < ISOWeekDate , D :: Error > { use serde_core :: de ; struct ISOWeekDateVisitor ; impl < 'de > de :: Visitor < 'de > for ISOWeekDateVisitor { type Value = ISOWeekDate ; fn expecting (& self , f : & mut core :: fmt :: Formatter ,) -> core :: fmt :: Result { f . write_str ("an ISO 8601 week date string") } # [inline] fn visit_bytes < E : de :: Error > (self , value : & [u8] ,) -> Result < ISOWeekDate , E > { DEFAULT_DATETIME_PARSER . parse_iso_week_date (value) . map_err (de :: Error :: custom) } # [inline] fn visit_str < E : de :: Error > (self , value : & str ,) -> Result < ISOWeekDate , E > { self . visit_bytes (value . as_bytes ()) } } deserializer . deserialize_str (ISOWeekDateVisitor) } }
};
}
