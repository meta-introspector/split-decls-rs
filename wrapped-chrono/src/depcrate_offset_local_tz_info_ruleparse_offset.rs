// Generated macro for parse_offset (function)
macro_rules! Depcrate_offset_local_tz_info_ruleparse_offset {
() => {
// Module: crate::offset::local::tz_info::rule
// Provides: {"parse_offset"}
// Dependencies: {}
# [doc = " Parse time zone offset"] fn parse_offset (cursor : & mut Cursor) -> Result < i32 , Error > { let (sign , hour , minute , second) = parse_signed_hhmmss (cursor) ? ; if ! (0 ..= 24) . contains (& hour) { return Err (Error :: InvalidTzString ("invalid offset hour")) ; } if ! (0 ..= 59) . contains (& minute) { return Err (Error :: InvalidTzString ("invalid offset minute")) ; } if ! (0 ..= 59) . contains (& second) { return Err (Error :: InvalidTzString ("invalid offset second")) ; } Ok (sign * (hour * 3600 + minute * 60 + second)) }
};
}
