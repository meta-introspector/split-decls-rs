// Generated macro for impl_112 (impl)
macro_rules! Depcrate_datetime_serdeimpl_112 {
() => {
// Module: crate::datetime::serde
// Provides: {"impl_112"}
// Dependencies: {}
# [doc = " Serialize to an RFC 3339 formatted string"] # [doc = ""] # [doc = " As an extension to RFC 3339 this can serialize `DateTime`s outside the range of 0-9999 years"] # [doc = " using an ISO 8601 syntax (which prepends an `-` or `+`)."] # [doc = ""] # [doc = " See [the `serde` module](crate::serde) for alternate serializations."] impl < Tz : TimeZone > ser :: Serialize for DateTime < Tz > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { struct FormatIso8601 < 'a , Tz : TimeZone > { inner : & 'a DateTime < Tz > , } impl < Tz : TimeZone > fmt :: Display for FormatIso8601 < '_ , Tz > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let naive = self . inner . naive_local () ; let offset = self . inner . offset . fix () ; write_rfc3339 (f , naive , offset , SecondsFormat :: AutoSi , true) } } serializer . collect_str (& FormatIso8601 { inner : self }) } }
};
}
