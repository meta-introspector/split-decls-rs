// Generated macro for macro_66 (macro)
macro_rules! Depcrate_ruremacro_66 {
() => {
// Module: crate::rure
// Provides: {"macro_66"}
// Dependencies: {}
ffi_fn ! { fn rure_compile_set (patterns : * const * const u8 , patterns_lengths : * const size_t , patterns_count : size_t , flags : u32 , options : * const Options , error : * mut Error) -> * const RegexSet { let (raw_pats , raw_patsl) = unsafe { (slice :: from_raw_parts (patterns , patterns_count) , slice :: from_raw_parts (patterns_lengths , patterns_count)) } ; let mut pats = Vec :: with_capacity (patterns_count) ; for (& raw_pat , & raw_patl) in raw_pats . iter () . zip (raw_patsl) { let pat = unsafe { slice :: from_raw_parts (raw_pat , raw_patl) } ; pats . push (match str :: from_utf8 (pat) { Ok (pat) => pat , Err (err) => { unsafe { if ! error . is_null () { * error = Error :: new (ErrorKind :: Str (err)) ; } return ptr :: null () ; } } }) ; } let mut builder = bytes :: RegexSetBuilder :: new (pats) ; if ! options . is_null () { let options = unsafe { &* options } ; builder . size_limit (options . size_limit) ; builder . dfa_size_limit (options . dfa_size_limit) ; } builder . case_insensitive (flags & RURE_FLAG_CASEI > 0) ; builder . multi_line (flags & RURE_FLAG_MULTI > 0) ; builder . dot_matches_new_line (flags & RURE_FLAG_DOTNL > 0) ; builder . swap_greed (flags & RURE_FLAG_SWAP_GREED > 0) ; builder . ignore_whitespace (flags & RURE_FLAG_SPACE > 0) ; builder . unicode (flags & RURE_FLAG_UNICODE > 0) ; match builder . build () { Ok (re) => { Box :: into_raw (Box :: new (RegexSet { re : re })) } Err (err) => { unsafe { if ! error . is_null () { * error = Error :: new (ErrorKind :: Regex (err)) } ptr :: null () } } } } }
};
}
