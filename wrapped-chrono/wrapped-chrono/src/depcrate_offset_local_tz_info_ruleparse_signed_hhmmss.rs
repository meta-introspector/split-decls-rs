// Generated macro for parse_signed_hhmmss (function)
macro_rules! Depcrate_offset_local_tz_info_ruleparse_signed_hhmmss {
() => {
// Module: crate::offset::local::tz_info::rule
// Provides: {"parse_signed_hhmmss"}
// Dependencies: {}
# [doc = " Parse signed hours, minutes and seconds"] fn parse_signed_hhmmss (cursor : & mut Cursor) -> Result < (i32 , i32 , i32 , i32) , Error > { let mut sign = 1 ; if let Some (& c) = cursor . peek () { if c == b'+' || c == b'-' { cursor . read_exact (1) ? ; if c == b'-' { sign = - 1 ; } } } let (hour , minute , second) = parse_hhmmss (cursor) ? ; Ok ((sign , hour , minute , second)) }
};
}
