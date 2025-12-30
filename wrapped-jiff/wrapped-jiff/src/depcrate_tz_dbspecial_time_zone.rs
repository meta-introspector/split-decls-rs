// Generated macro for special_time_zone (function)
macro_rules! Depcrate_tz_dbspecial_time_zone {
() => {
// Module: crate::tz::db
// Provides: {"special_time_zone"}
// Dependencies: {}
# [doc = " Checks if `name` is a \"special\" time zone and returns one if so."] # [doc = ""] # [doc = " This is limited to special constants that should have consistent values"] # [doc = " across time zone database implementations. For example, `UTC`."] fn special_time_zone (name : & str) -> Option < TimeZone > { if utf8 :: cmp_ignore_ascii_case ("utc" , name) . is_eq () { return Some (TimeZone :: UTC) ; } if utf8 :: cmp_ignore_ascii_case ("etc/unknown" , name) . is_eq () { return Some (TimeZone :: unknown ()) ; } None }
};
}
