// Generated macro for parse_value_from_args (function)
macro_rules! Depcrate_utils_shared_helpersparse_value_from_args {
() => {
// Module: crate::utils::shared_helpers
// Provides: {"parse_value_from_args"}
// Dependencies: {}
# [doc = " Finds `key` and returns its value from the given list of arguments `args`."] pub fn parse_value_from_args < 'a > (args : & 'a [OsString] , key : & str) -> Option < & 'a str > { let mut args = args . iter () ; while let Some (arg) = args . next () { let arg = arg . to_str () . unwrap () ; if let Some (value) = arg . strip_prefix (& format ! ("{key}=")) { return Some (value) ; } else if arg == key { return args . next () . map (| v | v . to_str () . unwrap ()) ; } } None }
};
}
