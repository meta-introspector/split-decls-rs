// Generated macro for impl_1374 (impl)
macro_rules! Depcrate_zonedimpl_1374 {
() => {
// Module: crate::zoned
// Provides: {"impl_1374"}
// Dependencies: {}
# [doc = " Converts a `Zoned` datetime into a human readable datetime string."] # [doc = ""] # [doc = " (This `Debug` representation currently emits the same string as the"] # [doc = " `Display` representation, but this is not a guarantee.)"] # [doc = ""] # [doc = " Options currently supported:"] # [doc = ""] # [doc = " * [`std::fmt::Formatter::precision`] can be set to control the precision"] # [doc = " of the fractional second component."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use jiff::civil::date;"] # [doc = ""] # [doc = " let zdt = date(2024, 6, 15).at(7, 0, 0, 123_000_000).in_tz(\"US/Eastern\")?;"] # [doc = " assert_eq!("] # [doc = "     format!(\"{zdt:.6?}\"),"] # [doc = "     \"2024-06-15T07:00:00.123000-04:00[US/Eastern]\","] # [doc = " );"] # [doc = " // Precision values greater than 9 are clamped to 9."] # [doc = " assert_eq!("] # [doc = "     format!(\"{zdt:.300?}\"),"] # [doc = "     \"2024-06-15T07:00:00.123000000-04:00[US/Eastern]\","] # [doc = " );"] # [doc = " // A precision of 0 implies the entire fractional"] # [doc = " // component is always truncated."] # [doc = " assert_eq!("] # [doc = "     format!(\"{zdt:.0?}\"),"] # [doc = "     \"2024-06-15T07:00:00-04:00[US/Eastern]\","] # [doc = " );"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] impl core :: fmt :: Debug for Zoned { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { core :: fmt :: Display :: fmt (self , f) } }
};
}
