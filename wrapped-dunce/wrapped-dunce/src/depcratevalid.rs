// Generated macro for valid (function)
macro_rules! Depcratevalid {
() => {
// Module: crate
// Provides: {"valid"}
// Dependencies: {}
# [test] fn valid () { assert ! (! is_valid_filename (".." . as_ref ())) ; assert ! (! is_valid_filename ("." . as_ref ())) ; assert ! (! is_valid_filename ("aaaaaaaaaa:" . as_ref ())) ; assert ! (! is_valid_filename ("ą:ą" . as_ref ())) ; assert ! (! is_valid_filename ("" . as_ref ())) ; assert ! (! is_valid_filename ("a " . as_ref ())) ; assert ! (! is_valid_filename (" a. " . as_ref ())) ; assert ! (! is_valid_filename ("a/" . as_ref ())) ; assert ! (! is_valid_filename ("/a" . as_ref ())) ; assert ! (! is_valid_filename ("/" . as_ref ())) ; assert ! (! is_valid_filename ("\\" . as_ref ())) ; assert ! (! is_valid_filename ("\\a" . as_ref ())) ; assert ! (! is_valid_filename ("<x>" . as_ref ())) ; assert ! (! is_valid_filename ("a*" . as_ref ())) ; assert ! (! is_valid_filename ("?x" . as_ref ())) ; assert ! (! is_valid_filename ("a\0a" . as_ref ())) ; assert ! (! is_valid_filename ("\x1f" . as_ref ())) ; assert ! (! is_valid_filename ("a" . repeat (257) . as_ref ())) ; assert ! (is_valid_filename ("®" . repeat (254) . as_ref ())) ; assert ! (is_valid_filename ("ファイル" . as_ref ())) ; assert ! (is_valid_filename ("a" . as_ref ())) ; assert ! (is_valid_filename ("a.aaaaaaaa" . as_ref ())) ; assert ! (is_valid_filename ("a........a" . as_ref ())) ; assert ! (is_valid_filename ("       b" . as_ref ())) ; }
};
}
