// Generated macro for impl_1031 (impl)
macro_rules! Depcrate_tz_posiximpl_1031 {
() => {
// Module: crate::tz::posix
// Provides: {"impl_1031"}
// Dependencies: {}
impl PosixTimeZone < Abbreviation > { # [doc = " Parse a IANA tzfile v3+ `TZ` string from the given bytes."] # [cfg (feature = "alloc")] pub (crate) fn parse (bytes : impl AsRef < [u8] > ,) -> Result < PosixTimeZoneOwned , Error > { let bytes = bytes . as_ref () ; let inner = shared :: PosixTimeZone :: parse (bytes . as_ref ()) . map_err (Error :: shared) . map_err (| e | { e . context (err ! ("invalid POSIX TZ string {:?}" , Bytes (bytes))) }) ? ; Ok (PosixTimeZone { inner }) } # [doc = " Like `parse`, but parses a POSIX TZ string from a prefix of the"] # [doc = " given input. And remaining input is returned."] # [cfg (feature = "alloc")] pub (crate) fn parse_prefix < 'b , B : AsRef < [u8] > + ? Sized + 'b > (bytes : & 'b B ,) -> Result < (PosixTimeZoneOwned , & 'b [u8]) , Error > { let bytes = bytes . as_ref () ; let (inner , remaining) = shared :: PosixTimeZone :: parse_prefix (bytes . as_ref ()) . map_err (Error :: shared) . map_err (| e | { e . context (err ! ("invalid POSIX TZ string {:?}" , Bytes (bytes))) }) ? ; Ok ((PosixTimeZone { inner } , remaining)) } # [doc = " Converts from the shared-but-internal API for use in proc macros."] # [cfg (feature = "alloc")] pub (crate) fn from_shared_owned (sh : shared :: PosixTimeZone < Abbreviation > ,) -> PosixTimeZoneOwned { PosixTimeZone { inner : sh } } }
};
}
