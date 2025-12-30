// Generated macro for Error (enum)
macro_rules! Depcrate_offset_local_tz_infoError {
() => {
// Module: crate::offset::local::tz_info
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Unified error type for everything in the crate"] # [derive (Debug)] pub (crate) enum Error { # [doc = " Date time error"] DateTime (& 'static str) , # [doc = " Local time type search error"] FindLocalTimeType (& 'static str) , # [doc = " Local time type error"] LocalTimeType (& 'static str) , # [doc = " Invalid slice for integer conversion"] InvalidSlice (& 'static str) , # [doc = " Invalid Tzif file"] InvalidTzFile (& 'static str) , # [doc = " Invalid TZ string"] InvalidTzString (& 'static str) , # [doc = " I/O error"] Io (io :: Error) , # [doc = " Out of range error"] OutOfRange (& 'static str) , # [doc = " Integer parsing error"] ParseInt (ParseIntError) , # [doc = " Date time projection error"] ProjectDateTime (& 'static str) , # [doc = " System time error"] SystemTime (SystemTimeError) , # [doc = " Time zone error"] TimeZone (& 'static str) , # [doc = " Transition rule error"] TransitionRule (& 'static str) , # [doc = " Unsupported Tzif file"] UnsupportedTzFile (& 'static str) , # [doc = " Unsupported TZ string"] UnsupportedTzString (& 'static str) , # [doc = " UTF-8 error"] Utf8 (Utf8Error) , }
};
}
