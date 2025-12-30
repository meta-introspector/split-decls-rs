// Generated macro for split_flags (function)
macro_rules! Depcratesplit_flags {
() => {
// Module: crate
// Provides: {"split_flags"}
// Dependencies: {}
# [doc = " Split output produced by pkg-config --cflags and / or --libs into separate flags."] # [doc = ""] # [doc = " Backslash in output is used to preserve literal meaning of following byte.  Different words are"] # [doc = " separated by unescaped space. Other whitespace characters generally should not occur unescaped"] # [doc = " at all, apart from the newline at the end of output. For compatibility with what others"] # [doc = " consumers of pkg-config output would do in this scenario, they are used here for splitting as"] # [doc = " well."] fn split_flags (output : & [u8]) -> Vec < String > { let mut word = Vec :: new () ; let mut words = Vec :: new () ; let mut escaped = false ; for & b in output { match b { _ if escaped => { escaped = false ; word . push (b) ; } b'\\' => escaped = true , b'\t' | b'\n' | b'\r' | b' ' => { if ! word . is_empty () { words . push (String :: from_utf8 (word) . unwrap ()) ; word = Vec :: new () ; } } _ => word . push (b) , } } if ! word . is_empty () { words . push (String :: from_utf8 (word) . unwrap ()) ; } words }
};
}
