// Generated macro for impl_733 (impl)
macro_rules! Depcrate_signed_durationimpl_733 {
() => {
// Module: crate::signed_duration
// Provides: {"impl_733"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de > serde_core :: Deserialize < 'de > for SignedDuration { # [inline] fn deserialize < D : serde_core :: Deserializer < 'de > > (deserializer : D ,) -> Result < SignedDuration , D :: Error > { use serde_core :: de ; struct SignedDurationVisitor ; impl < 'de > de :: Visitor < 'de > for SignedDurationVisitor { type Value = SignedDuration ; fn expecting (& self , f : & mut core :: fmt :: Formatter ,) -> core :: fmt :: Result { f . write_str ("a signed duration string") } # [inline] fn visit_bytes < E : de :: Error > (self , value : & [u8] ,) -> Result < SignedDuration , E > { parse_iso_or_friendly (value) . map_err (de :: Error :: custom) } # [inline] fn visit_str < E : de :: Error > (self , value : & str ,) -> Result < SignedDuration , E > { self . visit_bytes (value . as_bytes ()) } } deserializer . deserialize_str (SignedDurationVisitor) } }
};
}
