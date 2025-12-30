// Generated macro for PosixTzEnv (enum)
macro_rules! Depcrate_tz_posixPosixTzEnv {
() => {
// Module: crate::tz::posix
// Provides: {"PosixTzEnv"}
// Dependencies: {}
# [doc = " The result of parsing the POSIX `TZ` environment variable."] # [doc = ""] # [doc = " A `TZ` variable can either be a time zone string with an optional DST"] # [doc = " transition rule, or it can begin with a `:` followed by an arbitrary set of"] # [doc = " bytes that is implementation defined."] # [doc = ""] # [doc = " In practice, the content following a `:` is treated as an IANA time zone"] # [doc = " name. Moreover, even if the `TZ` string doesn't start with a `:` but"] # [doc = " corresponds to a IANA time zone name, then it is interpreted as such."] # [doc = " (See the module docs.) However, this type only encapsulates the choices"] # [doc = " strictly provided by POSIX: either a time zone string with an optional DST"] # [doc = " transition rule, or an implementation defined string with a `:` prefix. If,"] # [doc = " for example, `TZ=\"America/New_York\"`, then that case isn't encapsulated by"] # [doc = " this type. Callers needing that functionality will need to handle the error"] # [doc = " returned by parsing this type and layer their own semantics on top."] # [cfg (feature = "tz-system")] # [derive (Debug , Eq , PartialEq)] pub (crate) enum PosixTzEnv { # [doc = " A valid POSIX time zone with an optional DST transition rule."] Rule (PosixTimeZoneOwned) , # [doc = " An implementation defined string. This occurs when the `TZ` value"] # [doc = " starts with a `:`. The string returned here does not include the `:`."] Implementation (alloc :: boxed :: Box < str >) , }
};
}
