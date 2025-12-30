// Generated macro for parse_hhmmss (function)
macro_rules! Depcrate_offset_local_tz_info_ruleparse_hhmmss {
() => {
// Module: crate::offset::local::tz_info::rule
// Provides: {"parse_hhmmss"}
// Dependencies: {}
# [doc = " Parse hours, minutes and seconds"] fn parse_hhmmss (cursor : & mut Cursor) -> Result < (i32 , i32 , i32) , Error > { let hour = cursor . read_int () ? ; let mut minute = 0 ; let mut second = 0 ; if cursor . read_optional_tag (b":") ? { minute = cursor . read_int () ? ; if cursor . read_optional_tag (b":") ? { second = cursor . read_int () ? ; } } Ok ((hour , minute , second)) }
};
}
