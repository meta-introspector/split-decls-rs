// Generated macro for impl_868 (impl)
macro_rules! Depcrate_timestampimpl_868 {
() => {
// Module: crate::timestamp
// Provides: {"impl_868"}
// Dependencies: {}
# [doc = " Converts a `Timestamp` datetime into a human readable datetime string."] # [doc = ""] # [doc = " (This `Debug` representation currently emits the same string as the"] # [doc = " `Display` representation, but this is not a guarantee.)"] # [doc = ""] # [doc = " Options currently supported:"] # [doc = ""] # [doc = " * [`std::fmt::Formatter::precision`] can be set to control the precision"] # [doc = " of the fractional second component."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use jiff::Timestamp;"] # [doc = ""] # [doc = " let ts = Timestamp::new(1_123_456_789, 123_000_000)?;"] # [doc = " assert_eq!("] # [doc = "     format!(\"{ts:.6?}\"),"] # [doc = "     \"2005-08-07T23:19:49.123000Z\","] # [doc = " );"] # [doc = " // Precision values greater than 9 are clamped to 9."] # [doc = " assert_eq!("] # [doc = "     format!(\"{ts:.300?}\"),"] # [doc = "     \"2005-08-07T23:19:49.123000000Z\","] # [doc = " );"] # [doc = " // A precision of 0 implies the entire fractional"] # [doc = " // component is always truncated."] # [doc = " assert_eq!("] # [doc = "     format!(\"{ts:.0?}\"),"] # [doc = "     \"2005-08-07T23:19:49Z\","] # [doc = " );"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] impl core :: fmt :: Debug for Timestamp { # [inline] fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { core :: fmt :: Display :: fmt (self , f) } }
};
}
