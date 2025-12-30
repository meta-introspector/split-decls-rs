// Generated macro for sub_string (function)
macro_rules! Depcrate_utilsub_string {
() => {
// Module: crate::util
// Provides: {"sub_string"}
// Dependencies: {}
# [doc = " Return a substring of the given ANSIStrings sequence, while keeping the formatting."] pub fn sub_string (start : usize , len : usize , strs : & ANSIStrings) -> Vec < ANSIString < 'static > > { let mut vec = Vec :: new () ; let mut pos = start ; let mut len_rem = len ; for i in strs . 0 . iter () { let fragment = i . deref () ; let frag_len = fragment . len () ; if pos >= frag_len { pos -= frag_len ; continue ; } if len_rem == 0 { break ; } let end = pos + len_rem ; let pos_end = if end >= frag_len { frag_len } else { end } ; vec . push (i . style_ref () . paint (String :: from (& fragment [pos .. pos_end]))) ; if end <= frag_len { break ; } len_rem -= pos_end - pos ; pos = 0 ; } vec }
};
}
