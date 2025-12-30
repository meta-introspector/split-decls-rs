// Generated macro for split_args (function)
macro_rules! Depcrate_utilssplit_args {
() => {
// Module: crate::utils
// Provides: {"split_args"}
// Dependencies: {}
pub fn split_args (args : & str) -> Result < Vec < String > , String > { let mut out = Vec :: new () ; let mut start = 0 ; let args = args . trim () ; let mut iter = args . char_indices () . peekable () ; while let Some ((pos , c)) = iter . next () { if c == ' ' { out . push (args [start .. pos] . to_string ()) ; let mut found_start = false ; while let Some ((pos , c)) = iter . peek () { if * c != ' ' { start = * pos ; found_start = true ; break ; } else { iter . next () ; } } if ! found_start { return Ok (out) ; } } else if c == '"' || c == '\'' { let end = c ; let mut found_end = false ; while let Some ((_ , c)) = iter . next () { if c == end { found_end = true ; break ; } else if c == '\\' { iter . next () ; } } if ! found_end { return Err (format ! ("Didn't find `{}` at the end of `{}`" , end , & args [start ..])) ; } } else if c == '\\' { iter . next () ; } } let s = args [start ..] . trim () ; if ! s . is_empty () { out . push (s . to_string ()) ; } Ok (out) }
};
}
