// Generated macro for sys (module)
macro_rules! Depcrate_tz_systemsys {
() => {
// Module: crate::tz::system
// Provides: {"sys"}
// Dependencies: {}
# [cfg (not (any (unix , windows , all (feature = "js" , any (target_arch = "wasm32" , target_arch = "wasm64") , target_os = "unknown"))))] mod sys { use crate :: tz :: { TimeZone , TimeZoneDatabase } ; pub (super) fn get (_db : & TimeZoneDatabase) -> Option < TimeZone > { warn ! ("getting system time zone on this platform is unsupported") ; None } pub (super) fn read (_db : & TimeZoneDatabase , path : & str ,) -> Option < TimeZone > { match super :: read_unnamed_tzif_file (path) { Ok (tz) => Some (tz) , Err (_err) => { debug ! ("failed to read {path} as unnamed time zone: {_err}") ; None } } } }
};
}
