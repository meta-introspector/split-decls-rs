// Generated macro for len (function)
macro_rules! Depcratelen {
() => {
// Module: crate
// Provides: {"len"}
// Dependencies: {}
# [test] fn len () { assert_eq ! (1 , windows_char_len (OsStr :: new ("a"))) ; assert_eq ! (1 , windows_char_len (OsStr :: new ("€"))) ; assert_eq ! (1 , windows_char_len (OsStr :: new ("本"))) ; assert_eq ! (2 , windows_char_len (OsStr :: new ("🧐"))) ; assert_eq ! (2 , windows_char_len (OsStr :: new ("®®"))) ; }
};
}
