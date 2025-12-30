// Generated macro for impl_766 (impl)
macro_rules! Depcrate_spanimpl_766 {
() => {
// Module: crate::span
// Provides: {"impl_766"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de > serde_core :: Deserialize < 'de > for Span { # [inline] fn deserialize < D : serde_core :: Deserializer < 'de > > (deserializer : D ,) -> Result < Span , D :: Error > { use serde_core :: de ; struct SpanVisitor ; impl < 'de > de :: Visitor < 'de > for SpanVisitor { type Value = Span ; fn expecting (& self , f : & mut core :: fmt :: Formatter ,) -> core :: fmt :: Result { f . write_str ("a span duration string") } # [inline] fn visit_bytes < E : de :: Error > (self , value : & [u8] ,) -> Result < Span , E > { parse_iso_or_friendly (value) . map_err (de :: Error :: custom) } # [inline] fn visit_str < E : de :: Error > (self , value : & str) -> Result < Span , E > { self . visit_bytes (value . as_bytes ()) } } deserializer . deserialize_str (SpanVisitor) } }
};
}
