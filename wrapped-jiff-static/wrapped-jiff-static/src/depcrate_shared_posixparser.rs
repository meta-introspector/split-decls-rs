// Generated macro for Parser (struct)
macro_rules! Depcrate_shared_posixParser {
() => {
// Module: crate::shared::posix
// Provides: {"Parser"}
// Dependencies: {}
# [doc = " A parser for POSIX time zones."] # [derive (Debug)] struct Parser < 's > { # [doc = " The `TZ` string that we're parsing."] tz : & 's [u8] , # [doc = " The parser's current position in `tz`."] pos : core :: cell :: Cell < usize > , # [doc = " Whether to use IANA rules, i.e., when parsing a TZ string in a TZif"] # [doc = " file of version 3 or greater. From `tzfile(5)`:"] # [doc = ""] # [doc = " > First, the hours part of its transition times may be signed and range"] # [doc = " > from `-167` through `167` instead of the POSIX-required unsigned"] # [doc = " > values from `0` through `24`. Second, DST is in effect all year if"] # [doc = " > it starts January 1 at 00:00 and ends December 31 at 24:00 plus the"] # [doc = " > difference between daylight saving and standard time."] # [doc = ""] # [doc = " At time of writing, I don't think I understand the significance of"] # [doc = " the second part above. (RFC 8536 elaborates that it is meant to be an"] # [doc = " explicit clarification of something that POSIX itself implies.) But the"] # [doc = " first part is clear: it permits the hours to be a bigger range."] ianav3plus : bool , }
};
}
