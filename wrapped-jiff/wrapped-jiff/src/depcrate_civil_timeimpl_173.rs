// Generated macro for impl_173 (impl)
macro_rules! Depcrate_civil_timeimpl_173 {
() => {
// Module: crate::civil::time
// Provides: {"impl_173"}
// Dependencies: {}
# [doc = " Converts a `Time` into a human readable time string."] # [doc = ""] # [doc = " (This `Debug` representation currently emits the same string as the"] # [doc = " `Display` representation, but this is not a guarantee.)"] # [doc = ""] # [doc = " Options currently supported:"] # [doc = ""] # [doc = " * [`std::fmt::Formatter::precision`] can be set to control the precision"] # [doc = " of the fractional second component."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use jiff::civil::time;"] # [doc = ""] # [doc = " let t = time(7, 0, 0, 123_000_000);"] # [doc = " assert_eq!(format!(\"{t:.6?}\"), \"07:00:00.123000\");"] # [doc = " // Precision values greater than 9 are clamped to 9."] # [doc = " assert_eq!(format!(\"{t:.300?}\"), \"07:00:00.123000000\");"] # [doc = " // A precision of 0 implies the entire fractional"] # [doc = " // component is always truncated."] # [doc = " assert_eq!(format!(\"{t:.0?}\"), \"07:00:00\");"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] impl core :: fmt :: Debug for Time { # [inline] fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { core :: fmt :: Display :: fmt (self , f) } }
};
}
