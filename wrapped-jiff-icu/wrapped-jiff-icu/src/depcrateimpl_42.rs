// Generated macro for impl_42 (impl)
macro_rules! Depcrateimpl_42 {
() => {
// Module: crate
// Provides: {"impl_42"}
// Dependencies: {}
# [doc = " Converts from a [`&jiff::tz::TimeZone`](jiff::tz::TimeZone) to a"] # [doc = " [`icu_time::TimeZone`]."] # [cfg (feature = "zoned")] impl < 'a > ConvertFrom < & 'a JiffTimeZone > for IcuTimeZone { fn convert_from (v : & 'a JiffTimeZone) -> IcuTimeZone { let Some (iana_name) = v . iana_name () else { return IcuTimeZone :: UNKNOWN ; } ; icu_time :: zone :: iana :: IanaParser :: new () . parse (iana_name) } }
};
}
