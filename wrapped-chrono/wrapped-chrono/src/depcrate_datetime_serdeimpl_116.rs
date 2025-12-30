// Generated macro for impl_116 (impl)
macro_rules! Depcrate_datetime_serdeimpl_116 {
() => {
// Module: crate::datetime::serde
// Provides: {"impl_116"}
// Dependencies: {}
# [doc = " Deserialize an RFC 3339 formatted string into a `DateTime<Utc>`"] # [doc = ""] # [doc = " If the value contains an offset from UTC that is not zero, the value will be converted to UTC."] # [doc = ""] # [doc = " As an extension to RFC 3339 this can deserialize to `DateTime`s outside the range of 0-9999"] # [doc = " years using an ISO 8601 syntax (which prepends an `-` or `+`)."] # [doc = ""] # [doc = " See [the `serde` module](crate::serde) for alternate deserialization formats."] impl < 'de > de :: Deserialize < 'de > for DateTime < Utc > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { deserializer . deserialize_str (DateTimeVisitor) . map (| dt | dt . with_timezone (& Utc)) } }
};
}
