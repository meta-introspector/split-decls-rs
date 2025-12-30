// Generated macro for try_print_as_readable (function)
macro_rules! Depcrate_h3try_print_as_readable {
() => {
// Module: crate::h3
// Provides: {"try_print_as_readable"}
// Dependencies: {}
fn try_print_as_readable (hdr : & [u8] , f : & mut fmt :: Formatter) -> fmt :: Result { match std :: str :: from_utf8 (hdr) { Ok (s) => f . write_str (& s . escape_default () . to_string ()) , Err (_) => write ! (f , "{hdr:?}") , } }
};
}
