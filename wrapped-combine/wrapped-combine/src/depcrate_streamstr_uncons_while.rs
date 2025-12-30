// Generated macro for str_uncons_while (function)
macro_rules! Depcrate_streamstr_uncons_while {
() => {
// Module: crate::stream
// Provides: {"str_uncons_while"}
// Dependencies: {}
# [allow (clippy :: while_let_loop)] fn str_uncons_while < 'a , F > (slice : & mut & 'a str , mut chars : Chars < 'a > , mut f : F) -> & 'a str where F : FnMut (char) -> bool , { let mut last_char_size = 0 ; macro_rules ! test_next { () => { match chars . next () { Some (c) => { if ! f (c) { last_char_size = c . len_utf8 () ; break ; } } None => break , } } ; } loop { test_next ! () ; test_next ! () ; test_next ! () ; test_next ! () ; test_next ! () ; test_next ! () ; test_next ! () ; test_next ! () ; } let len = slice . len () - chars . as_str () . len () - last_char_size ; let (result , rest) = slice . split_at (len) ; * slice = rest ; result }
};
}
