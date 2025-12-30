// Generated macro for rure_escape (function)
macro_rules! Depcrate_rurerure_escape {
() => {
// Module: crate::rure
// Provides: {"rure_escape"}
// Dependencies: {}
# [doc = " A helper function that implements fallible escaping in a way that returns"] # [doc = " an error if escaping failed."] # [doc = ""] # [doc = " This should ideally be exposed, but it needs API design work. In"] # [doc = " particular, this should not return a C string, but a `const uint8_t *`"] # [doc = " instead, since it may contain a NUL byte."] fn rure_escape (pattern : * const u8 , length : size_t , error : * mut Error ,) -> * const c_char { let pat : & [u8] = unsafe { slice :: from_raw_parts (pattern , length) } ; let str_pat = match str :: from_utf8 (pat) { Ok (val) => val , Err (err) => unsafe { if ! error . is_null () { * error = Error :: new (ErrorKind :: Str (err)) ; } return ptr :: null () ; } , } ; let esc_pat = regex :: escape (str_pat) ; let c_esc_pat = match CString :: new (esc_pat) { Ok (val) => val , Err (err) => unsafe { if ! error . is_null () { * error = Error :: new (ErrorKind :: Nul (err)) ; } return ptr :: null () ; } , } ; c_esc_pat . into_raw () as * const c_char }
};
}
