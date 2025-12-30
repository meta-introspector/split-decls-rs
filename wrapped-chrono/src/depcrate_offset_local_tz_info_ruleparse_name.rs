// Generated macro for parse_name (function)
macro_rules! Depcrate_offset_local_tz_info_ruleparse_name {
() => {
// Module: crate::offset::local::tz_info::rule
// Provides: {"parse_name"}
// Dependencies: {}
# [doc = " Parse time zone name"] fn parse_name < 'a > (cursor : & mut Cursor < 'a >) -> Result < & 'a [u8] , Error > { match cursor . peek () { Some (b'<') => { } _ => return Ok (cursor . read_while (u8 :: is_ascii_alphabetic) ?) , } cursor . read_exact (1) ? ; let unquoted = cursor . read_until (| & x | x == b'>') ? ; cursor . read_exact (1) ? ; Ok (unquoted) }
};
}
