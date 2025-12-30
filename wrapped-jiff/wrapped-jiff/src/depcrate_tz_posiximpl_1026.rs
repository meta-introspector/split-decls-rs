// Generated macro for impl_1026 (impl)
macro_rules! Depcrate_tz_posiximpl_1026 {
() => {
// Module: crate::tz::posix
// Provides: {"impl_1026"}
// Dependencies: {}
# [cfg (feature = "tz-system")] impl PosixTzEnv { # [doc = " Parse a POSIX `TZ` environment variable string from the given bytes."] fn parse (bytes : impl AsRef < [u8] >) -> Result < PosixTzEnv , Error > { let bytes = bytes . as_ref () ; if bytes . get (0) == Some (& b':') { let Ok (string) = core :: str :: from_utf8 (& bytes [1 ..]) else { return Err (err ! ("POSIX time zone string with a ':' prefix contains \
                     invalid UTF-8: {:?}" , Bytes (& bytes [1 ..]) ,)) ; } ; Ok (PosixTzEnv :: Implementation (string . into ())) } else { PosixTimeZone :: parse (bytes) . map (PosixTzEnv :: Rule) } } # [doc = " Parse a POSIX `TZ` environment variable string from the given `OsStr`."] pub (crate) fn parse_os_str (osstr : impl AsRef < std :: ffi :: OsStr > ,) -> Result < PosixTzEnv , Error > { PosixTzEnv :: parse (parse :: os_str_bytes (osstr . as_ref ()) ?) } }
};
}
