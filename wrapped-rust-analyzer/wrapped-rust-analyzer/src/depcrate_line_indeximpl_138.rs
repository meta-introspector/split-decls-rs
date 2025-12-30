// Generated macro for impl_138 (impl)
macro_rules! Depcrate_line_indeximpl_138 {
() => {
// Module: crate::line_index
// Provides: {"impl_138"}
// Dependencies: {}
impl LineEndings { # [doc = " Replaces `\\r\\n` with `\\n` in-place in `src`."] pub (crate) fn normalize (src : String) -> (String , LineEndings) { let mut buf = src . into_bytes () ; let mut gap_len = 0 ; let mut tail = buf . as_mut_slice () ; let mut crlf_seen = false ; let finder = memmem :: Finder :: new (b"\r\n") ; loop { let idx = match finder . find (& tail [gap_len ..]) { None if crlf_seen => tail . len () , None => return (unsafe { String :: from_utf8_unchecked (buf) } , LineEndings :: Unix) , Some (idx) => { crlf_seen = true ; idx + gap_len } } ; tail . copy_within (gap_len .. idx , 0) ; tail = & mut tail [idx - gap_len ..] ; if tail . len () == gap_len { break ; } gap_len += 1 ; } let new_len = buf . len () - gap_len ; let src = unsafe { buf . set_len (new_len) ; String :: from_utf8_unchecked (buf) } ; (src , LineEndings :: Dos) } }
};
}
