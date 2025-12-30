// Generated macro for tests (module)
macro_rules! Depcrate_sourcetests {
() => {
// Module: crate::source
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: Source ; # [test] fn simple_append () { let mut s = Source :: default () ; s . push_str ("x") ; assert_eq ! (s . s , "x") ; s . push_str ("y") ; assert_eq ! (s . s , "xy") ; s . push_str ("z ") ; assert_eq ! (s . s , "xyz ") ; s . push_str (" a ") ; assert_eq ! (s . s , "xyz  a ") ; s . push_str ("\na") ; assert_eq ! (s . s , "xyz  a \na") ; } # [test] fn newline_remap () { let mut s = Source :: default () ; s . push_str ("function() {\n") ; s . push_str ("y\n") ; s . push_str ("}\n") ; assert_eq ! (s . s , "function() {\n  y\n}\n") ; } # [test] fn if_else () { let mut s = Source :: default () ; s . push_str ("if() {\n") ; s . push_str ("y\n") ; s . push_str ("} else if () {\n") ; s . push_str ("z\n") ; s . push_str ("}\n") ; assert_eq ! (s . s , "if() {\n  y\n} else if () {\n  z\n}\n") ; } # [test] fn trim_ws () { let mut s = Source :: default () ; s . push_str ("function() {
                x
        }" ,) ; assert_eq ! (s . s , "function() {\n  x\n}") ; } }
};
}
