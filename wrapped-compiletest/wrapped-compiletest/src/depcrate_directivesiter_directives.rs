// Generated macro for iter_directives (function)
macro_rules! Depcrate_directivesiter_directives {
() => {
// Module: crate::directives
// Provides: {"iter_directives"}
// Dependencies: {}
fn iter_directives (mode : TestMode , poisoned : & mut bool , testfile : & Utf8Path , rdr : impl Read , it : & mut dyn FnMut (DirectiveLine < '_ >) ,) { if testfile . is_dir () { return ; } if mode == TestMode :: CoverageRun { let extra_directives : & [& str] = & ["needs-profiler-runtime" , "ignore-cross-compile" ,] ; for raw_directive in extra_directives { it (DirectiveLine { line_number : 0 , revision : None , raw_directive }) ; } } let mut rdr = BufReader :: with_capacity (1024 , rdr) ; let mut ln = String :: new () ; let mut line_number = 0 ; loop { line_number += 1 ; ln . clear () ; if rdr . read_line (& mut ln) . unwrap () == 0 { break ; } let ln = ln . trim () ; let Some (directive_line) = line_directive (line_number , ln) else { continue ; } ; if testfile . extension () == Some ("rs") { let CheckDirectiveResult { is_known_directive , trailing_directive } = check_directive (directive_line . raw_directive , mode) ; if ! is_known_directive { * poisoned = true ; error ! ("{testfile}:{line_number}: detected unknown compiletest test directive `{}`" , directive_line . raw_directive ,) ; return ; } if let Some (trailing_directive) = & trailing_directive { * poisoned = true ; error ! ("{testfile}:{line_number}: detected trailing compiletest test directive `{}`" , trailing_directive ,) ; help ! ("put the trailing directive in its own line: `//@ {}`" , trailing_directive) ; return ; } } it (directive_line) ; } }
};
}
