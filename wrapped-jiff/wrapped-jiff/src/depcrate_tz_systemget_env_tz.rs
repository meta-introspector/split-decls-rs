// Generated macro for get_env_tz (function)
macro_rules! Depcrate_tz_systemget_env_tz {
() => {
// Module: crate::tz::system
// Provides: {"get_env_tz"}
// Dependencies: {}
# [doc = " Materializes a `TimeZone` from a `TZ` environment variable."] # [doc = ""] # [doc = " Basically, `TZ` is usually just an IANA Time Zone Database name like"] # [doc = " `TZ=America/New_York` or `TZ=UTC`. But it can also be a POSIX time zone"] # [doc = " transition string like `TZ=EST5EDTM3.2.0,M11.1.0` or it can be a file path"] # [doc = " (absolute or relative) to a TZif file."] # [doc = ""] # [doc = " We try very hard to extract a time zone name from `TZ` and use that to look"] # [doc = " it up via `TimeZoneDatabase`. But we will fall back to unnamed TZif"] # [doc = " `TimeZone` if necessary."] # [doc = ""] # [doc = " This routine only returns `Ok(None)` when `TZ` is not set. If it is set"] # [doc = " but a `TimeZone` could not be extracted, then an error is returned."] fn get_env_tz (db : & TimeZoneDatabase) -> Result < Option < TimeZone > , Error > { let Some (tzenv) = std :: env :: var_os ("TZ") else { return Ok (None) } ; if tzenv . is_empty () { debug ! ("TZ environment variable set to empty value, \
             assuming TZ=UTC in order to conform to \
             widespread convention among Unix tooling" ,) ; return Ok (Some (TimeZone :: UTC)) ; } let tz_name_or_path = match PosixTzEnv :: parse_os_str (& tzenv) { Err (_err) => { debug ! ("failed to parse {tzenv:?} as POSIX TZ rule \
                 (attempting to treat it as an IANA time zone): {_err}" ,) ; tzenv . to_str () . ok_or_else (| | { err ! ("failed to parse {tzenv:?} as a POSIX TZ transition \
                         string, or as valid UTF-8" ,) }) ? . to_string () } Ok (PosixTzEnv :: Implementation (string)) => string . to_string () , Ok (PosixTzEnv :: Rule (tz)) => { return Ok (Some (TimeZone :: from_posix_tz (tz))) } } ; let needle = "zoneinfo/" ; let Some (rpos) = tz_name_or_path . rfind (needle) else { debug ! ("could not find {needle:?} in TZ={tz_name_or_path:?}, \
             therefore attempting lookup in {db:?}" ,) ; return match db . get (& tz_name_or_path) { Ok (tz) => Ok (Some (tz)) , Err (_err) => { debug ! ("using TZ={tz_name_or_path:?} as time zone name failed, \
                     could not find time zone in zoneinfo database {db:?} \
                     (continuing to try and read `{tz_name_or_path}` as \
                      a TZif file)" ,) ; sys :: read (db , & tz_name_or_path) . ok_or_else (| | { err ! ("failed to read TZ={tz_name_or_path:?} \
                             as a TZif file after attempting a tzdb \
                             lookup for `{tz_name_or_path}`" ,) }) . map (Some) } } ; } ; let name = & tz_name_or_path [rpos + needle . len () ..] ; debug ! ("extracted {name:?} from TZ={tz_name_or_path:?} \
         and assuming it is an IANA time zone name" ,) ; match db . get (& name) { Ok (tz) => return Ok (Some (tz)) , Err (_err) => { debug ! ("using {name:?} from TZ={tz_name_or_path:?}, \
                 could not find time zone in zoneinfo database {db:?} \
                 (continuing to try and use {tz_name_or_path:?})" ,) ; } } sys :: read (db , & tz_name_or_path) . ok_or_else (| | { err ! ("failed to read TZ={tz_name_or_path:?} \
                 as a TZif file after attempting a tzdb \
                 lookup for `{name}`" ,) }) . map (Some) }
};
}
