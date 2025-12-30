// Generated macro for macro_31 (macro)
macro_rules! Depcrate_ruremacro_31 {
() => {
// Module: crate::rure
// Provides: {"macro_31"}
// Dependencies: {}
ffi_fn ! { fn rure_compile (pattern : * const u8 , length : size_t , flags : u32 , options : * const Options , error : * mut Error ,) -> * const Regex { let pat = unsafe { slice :: from_raw_parts (pattern , length) } ; let pat = match str :: from_utf8 (pat) { Ok (pat) => pat , Err (err) => { unsafe { if ! error . is_null () { * error = Error :: new (ErrorKind :: Str (err)) ; } return ptr :: null () ; } } } ; let mut builder = bytes :: RegexBuilder :: new (pat) ; if ! options . is_null () { let options = unsafe { &* options } ; builder = builder . size_limit (options . size_limit) ; builder = builder . dfa_size_limit (options . dfa_size_limit) ; } builder = builder . case_insensitive (flags & RURE_FLAG_CASEI > 0) ; builder = builder . multi_line (flags & RURE_FLAG_MULTI > 0) ; builder = builder . dot_matches_new_line (flags & RURE_FLAG_DOTNL > 0) ; builder = builder . swap_greed (flags & RURE_FLAG_SWAP_GREED > 0) ; builder = builder . ignore_whitespace (flags & RURE_FLAG_SPACE > 0) ; builder = builder . unicode (flags & RURE_FLAG_UNICODE > 0) ; match builder . compile () { Ok (re) => { let mut capture_names = HashMap :: new () ; for (i , name) in re . capture_names () . enumerate () { if let Some (name) = name { capture_names . insert (name . to_owned () , i as i32) ; } } let re = Regex { re : re , capture_names : capture_names , } ; Box :: into_raw (Box :: new (re)) } Err (err) => { unsafe { if ! error . is_null () { * error = Error :: new (ErrorKind :: Regex (err)) ; } ptr :: null () } } } } }
};
}
