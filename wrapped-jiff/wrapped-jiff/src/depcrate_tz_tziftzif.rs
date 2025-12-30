// Generated macro for Tzif (struct)
macro_rules! Depcrate_tz_tzifTzif {
() => {
// Module: crate::tz::tzif
// Provides: {"Tzif"}
// Dependencies: {}
# [doc = " A time zone based on IANA TZif formatted data."] # [doc = ""] # [doc = " TZif is a binary format described by RFC 8536. Its typical structure is to"] # [doc = " define a single time zone per file in the `/usr/share/zoneinfo` directory"] # [doc = " on Unix systems. The name of a time zone is its file path with the"] # [doc = " `/usr/share/zoneinfo/` prefix stripped from it."] # [doc = ""] # [doc = " This type doesn't provide any facilities for dealing with files on disk"] # [doc = " or the `/usr/share/zoneinfo` directory. This type is just for parsing the"] # [doc = " contents of TZif formatted data in memory, and turning it into a data type"] # [doc = " that can be used as a time zone."] # [derive (Debug)] # [doc (hidden)] # [repr (align (8))] pub struct Tzif < STR , ABBREV , TYPES , TIMESTAMPS , STARTS , ENDS , INFOS > { inner : shared :: Tzif < STR , ABBREV , TYPES , TIMESTAMPS , STARTS , ENDS , INFOS > , # [doc = " The POSIX time zone for this TZif data, if present."] # [doc = ""] # [doc = " Note that this is also present on `shared::Tzif`, but uses the"] # [doc = " `shared::PosixTimeZone` type, which isn't quite what we want here."] # [doc = ""] # [doc = " For now we just duplicate it, which is slightly unfortunate. But this"] # [doc = " is small and not a huge deal. Ideally we can clean this up later."] posix_tz : Option < PosixTimeZone < ABBREV > > , }
};
}
