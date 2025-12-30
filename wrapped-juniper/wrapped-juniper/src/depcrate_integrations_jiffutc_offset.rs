// Generated macro for utc_offset (module)
macro_rules! Depcrate_integrations_jiffutc_offset {
() => {
// Module: crate::integrations::jiff
// Provides: {"utc_offset"}
// Dependencies: {}
mod utc_offset { use std :: fmt :: { self , Display } ; use jiff :: fmt :: { StdFmtWrite , strtime :: BrokenDownTime } ; use super :: UtcOffset ; # [doc = " Format of a [`UtcOffset` scalar][1]."] # [doc = ""] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/utc-offset"] const FORMAT : & str = "%:z" ; pub (super) fn utc_offset_from_str (value : & str) -> Result < jiff :: tz :: Offset , jiff :: Error > { let tm = BrokenDownTime :: parse (FORMAT , value) ? ; let offset = tm . offset () . expect ("successful %:z parsing guarantees offset") ; Ok (offset) } pub (super) fn to_output (v : & UtcOffset) -> impl Display { struct LazyFmt (BrokenDownTime) ; impl Display for LazyFmt { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . format (FORMAT , StdFmtWrite (f)) . map_err (| _ | fmt :: Error) } } LazyFmt (BrokenDownTime :: from (& jiff :: Zoned :: now () . with_time_zone (jiff :: tz :: TimeZone :: fixed (* v)) ,)) } pub (super) fn from_input (s : & str) -> Result < UtcOffset , Box < str > > { utc_offset_from_str (s) . map_err (| e | format ! ("Invalid `UtcOffset`: {e}") . into ()) } }
};
}
