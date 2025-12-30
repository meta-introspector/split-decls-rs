// Generated macro for macro_57 (macro)
macro_rules! Depcrate_ruremacro_57 {
() => {
// Module: crate::rure
// Provides: {"macro_57"}
// Dependencies: {}
ffi_fn ! { fn rure_iter_next_captures (it : * mut Iter , haystack : * const u8 , len : size_t , captures : * mut Captures ,) -> bool { let it = unsafe { & mut * it } ; let re = unsafe { &* it . re } ; let slots = unsafe { & mut (* captures) . 0 } ; let text = unsafe { slice :: from_raw_parts (haystack , len) } ; if it . last_end > text . len () { return false ; } let (s , e) = match re . captures_read_at (slots , text , it . last_end) { None => return false , Some (m) => (m . start () , m . end ()) , } ; if s == e { it . last_end += 1 ; if Some (e) == it . last_match { return rure_iter_next_captures (it , haystack , len , captures) ; } } else { it . last_end = e ; } it . last_match = Some (e) ; true } }
};
}
