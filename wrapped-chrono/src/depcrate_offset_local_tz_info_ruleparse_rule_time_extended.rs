// Generated macro for parse_rule_time_extended (function)
macro_rules! Depcrate_offset_local_tz_info_ruleparse_rule_time_extended {
() => {
// Module: crate::offset::local::tz_info::rule
// Provides: {"parse_rule_time_extended"}
// Dependencies: {}
# [doc = " Parse transition rule time with TZ string extensions"] fn parse_rule_time_extended (cursor : & mut Cursor) -> Result < i32 , Error > { let (sign , hour , minute , second) = parse_signed_hhmmss (cursor) ? ; if ! (- 167 ..= 167) . contains (& hour) { return Err (Error :: InvalidTzString ("invalid day time hour")) ; } if ! (0 ..= 59) . contains (& minute) { return Err (Error :: InvalidTzString ("invalid day time minute")) ; } if ! (0 ..= 59) . contains (& second) { return Err (Error :: InvalidTzString ("invalid day time second")) ; } Ok (sign * (hour * 3600 + minute * 60 + second)) }
};
}
