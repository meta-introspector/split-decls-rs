// Generated macro for impl_341 (impl)
macro_rules! Depcrate_fmt_friendly_printerimpl_341 {
() => {
// Module: crate::fmt::friendly::printer
// Provides: {"impl_341"}
// Dependencies: {}
impl Direction { # [doc = " Returns the sign string to use (as either a prefix or a suffix) based"] # [doc = " on the given parameters."] # [doc = ""] # [doc = " This lets us do the case analysis for how to write the sign exactly"] # [doc = " once."] fn sign (self , printer : & SpanPrinter , has_calendar : bool , signum : i8 ,) -> Option < DirectionSign > { match self { Direction :: Auto => match printer . spacing { Spacing :: None => { if signum < 0 { Some (DirectionSign :: Prefix ("-")) } else { None } } Spacing :: BetweenUnits | Spacing :: BetweenUnitsAndDesignators => { if signum < 0 { if printer . hms && ! has_calendar { Some (DirectionSign :: Prefix ("-")) } else { Some (DirectionSign :: Suffix (" ago")) } } else { None } } } , Direction :: Sign => { if signum < 0 { Some (DirectionSign :: Prefix ("-")) } else { None } } Direction :: ForceSign => { Some (DirectionSign :: Prefix (if signum < 0 { "-" } else { "+" })) } Direction :: Suffix => { if signum < 0 { Some (DirectionSign :: Suffix (" ago")) } else { None } } } } }
};
}
