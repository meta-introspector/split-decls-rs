// Generated macro for impl_41 (impl)
macro_rules! Depcrate_lineimpl_41 {
() => {
// Module: crate::line
// Provides: {"impl_41"}
// Dependencies: {}
impl < 'a > Line < 'a > { # [doc = " Attempt to parse this line, returning a `Line` depending on what"] # [doc = " type of line it was, or an `Error` if it couldn't be parsed."] pub fn new (input : & 'a str) -> Result < Line < 'a > , Error > { let input = match input . split_once ('#') { Some ((input , _)) => input , None => input , } ; if input . trim () . is_empty () { return Ok (Line :: Space) ; } if input . starts_with ("Zone") { return Ok (Line :: Zone (Zone :: from_str (input) ?)) ; } if input . starts_with (& [' ' , '\t'] [..]) { return Ok (Line :: Continuation (ZoneInfo :: from_iter (input . split_ascii_whitespace () ,) ?)) ; } if input . starts_with ("Rule") { return Ok (Line :: Rule (Rule :: from_str (input) ?)) ; } if input . starts_with ("Link") { return Ok (Line :: Link (Link :: from_str (input) ?)) ; } Err (Error :: InvalidLineType (input . to_string ())) } }
};
}
