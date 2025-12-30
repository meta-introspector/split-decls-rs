// Generated macro for validate_id (function)
macro_rules! Depcratevalidate_id {
() => {
// Module: crate
// Provides: {"validate_id"}
// Dependencies: {}
# [doc = " Validate the password hash identifier is well-formed."] # [doc = ""] # [doc = " Allowed characters match the regex: `[a-z0-9\\-]`, where the first and last characters do NOT"] # [doc = " contain a `-`."] fn validate_id (id : & str) -> Result < () > { let first = id . chars () . next () . ok_or (Error { }) ? ; let last = id . chars () . last () . ok_or (Error { }) ? ; for c in [first , last] { match c { 'a' ..= 'z' | '0' ..= '9' => () , _ => return Err (Error { }) , } } for c in id . chars () { match c { 'a' ..= 'z' | '0' ..= '9' | '-' => () , _ => return Err (Error { }) , } } Ok (()) }
};
}
