// Generated macro for impl_1408 (impl)
macro_rules! Depcrate_zonedimpl_1408 {
() => {
// Module: crate::zoned
// Provides: {"impl_1408"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de > serde_core :: Deserialize < 'de > for Zoned { # [inline] fn deserialize < D : serde_core :: Deserializer < 'de > > (deserializer : D ,) -> Result < Zoned , D :: Error > { use serde_core :: de ; struct ZonedVisitor ; impl < 'de > de :: Visitor < 'de > for ZonedVisitor { type Value = Zoned ; fn expecting (& self , f : & mut core :: fmt :: Formatter ,) -> core :: fmt :: Result { f . write_str ("a zoned datetime string") } # [inline] fn visit_bytes < E : de :: Error > (self , value : & [u8] ,) -> Result < Zoned , E > { DEFAULT_DATETIME_PARSER . parse_zoned (value) . map_err (de :: Error :: custom) } # [inline] fn visit_str < E : de :: Error > (self , value : & str) -> Result < Zoned , E > { self . visit_bytes (value . as_bytes ()) } } deserializer . deserialize_str (ZonedVisitor) } }
};
}
