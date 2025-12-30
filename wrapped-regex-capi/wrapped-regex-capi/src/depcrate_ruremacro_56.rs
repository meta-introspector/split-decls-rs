// Generated macro for macro_56 (macro)
macro_rules! Depcrate_ruremacro_56 {
() => {
// Module: crate::rure
// Provides: {"macro_56"}
// Dependencies: {}
ffi_fn ! { fn rure_iter_next (it : * mut Iter , haystack : * const u8 , len : size_t , match_info : * mut rure_match ,) -> bool { let it = unsafe { & mut * it } ; let re = unsafe { &* it . re } ; let text = unsafe { slice :: from_raw_parts (haystack , len) } ; if it . last_end > text . len () { return false ; } let (s , e) = match re . find_at (text , it . last_end) { None => return false , Some (m) => (m . start () , m . end ()) , } ; if s == e { it . last_end += 1 ; if Some (e) == it . last_match { return rure_iter_next (it , haystack , len , match_info) ; } } else { it . last_end = e ; } it . last_match = Some (e) ; if ! match_info . is_null () { unsafe { (* match_info) . start = s ; (* match_info) . end = e ; } } true } }
};
}
