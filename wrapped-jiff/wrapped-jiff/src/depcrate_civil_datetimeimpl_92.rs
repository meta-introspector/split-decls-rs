// Generated macro for impl_92 (impl)
macro_rules! Depcrate_civil_datetimeimpl_92 {
() => {
// Module: crate::civil::datetime
// Provides: {"impl_92"}
// Dependencies: {}
# [doc = " Converts a `DateTime` into a human readable datetime string."] # [doc = ""] # [doc = " (This `Debug` representation currently emits the same string as the"] # [doc = " `Display` representation, but this is not a guarantee.)"] # [doc = ""] # [doc = " Options currently supported:"] # [doc = ""] # [doc = " * [`std::fmt::Formatter::precision`] can be set to control the precision"] # [doc = " of the fractional second component."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use jiff::civil::date;"] # [doc = ""] # [doc = " let dt = date(2024, 6, 15).at(7, 0, 0, 123_000_000);"] # [doc = " assert_eq!(format!(\"{dt:.6?}\"), \"2024-06-15T07:00:00.123000\");"] # [doc = " // Precision values greater than 9 are clamped to 9."] # [doc = " assert_eq!(format!(\"{dt:.300?}\"), \"2024-06-15T07:00:00.123000000\");"] # [doc = " // A precision of 0 implies the entire fractional"] # [doc = " // component is always truncated."] # [doc = " assert_eq!(format!(\"{dt:.0?}\"), \"2024-06-15T07:00:00\");"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] impl core :: fmt :: Debug for DateTime { # [inline] fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { core :: fmt :: Display :: fmt (self , f) } }
};
}
