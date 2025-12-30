// Generated macro for impl_115 (impl)
macro_rules! Depcrate_datetime_serdeimpl_115 {
() => {
// Module: crate::datetime::serde
// Provides: {"impl_115"}
// Dependencies: {}
# [doc = " Deserialize an RFC 3339 formatted string into a `DateTime<FixedOffset>`"] # [doc = ""] # [doc = " As an extension to RFC 3339 this can deserialize to `DateTime`s outside the range of 0-9999"] # [doc = " years using an ISO 8601 syntax (which prepends an `-` or `+`)."] # [doc = ""] # [doc = " See [the `serde` module](crate::serde) for alternate deserialization formats."] impl < 'de > de :: Deserialize < 'de > for DateTime < FixedOffset > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { deserializer . deserialize_str (DateTimeVisitor) } }
};
}
