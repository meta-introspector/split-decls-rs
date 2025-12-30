// Generated macro for load_errors (function)
macro_rules! Depcrate_errorsload_errors {
() => {
// Module: crate::errors
// Provides: {"load_errors"}
// Dependencies: {}
# [doc = " Looks for either \"//~| KIND MESSAGE\" or \"//~^^... KIND MESSAGE\""] # [doc = " The former is a \"follow\" that inherits its target from the preceding line;"] # [doc = " the latter is an \"adjusts\" that goes that many lines up."] # [doc = ""] # [doc = " Goal is to enable tests both like: //~^^^ ERROR go up three"] # [doc = " and also //~^ ERROR message one for the preceding line, and"] # [doc = "          //~| ERROR message two for that same line."] # [doc = ""] # [doc = " If revision is not None, then we look"] # [doc = " for `//[X]~` instead, where `X` is the current revision."] pub fn load_errors (testfile : & Utf8Path , revision : Option < & str >) -> Vec < Error > { let rdr = BufReader :: new (File :: open (testfile . as_std_path ()) . unwrap ()) ; let mut last_nonfollow_error = None ; rdr . lines () . enumerate () . filter (| (_ , line) | line . is_ok ()) . filter_map (| (line_num , line) | { parse_expected (last_nonfollow_error , line_num + 1 , & line . unwrap () , revision) . map (| (follow_prev , error) | { if ! follow_prev { last_nonfollow_error = error . line_num ; } error } ,) }) . collect () }
};
}
