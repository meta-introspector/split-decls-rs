// Generated macro for impl_1376 (impl)
macro_rules! Depcrate_zonedimpl_1376 {
() => {
// Module: crate::zoned
// Provides: {"impl_1376"}
// Dependencies: {}
# [doc = " Parses a zoned timestamp from the Temporal datetime format."] # [doc = ""] # [doc = " See the [`fmt::temporal`](crate::fmt::temporal) for more information on"] # [doc = " the precise format."] # [doc = ""] # [doc = " Note that this is only enabled when the `std` feature"] # [doc = " is enabled because it requires access to a global"] # [doc = " [`TimeZoneDatabase`](crate::tz::TimeZoneDatabase)."] impl core :: str :: FromStr for Zoned { type Err = Error ; fn from_str (string : & str) -> Result < Zoned , Error > { DEFAULT_DATETIME_PARSER . parse_zoned (string) } }
};
}
