// Generated macro for string (function)
macro_rules! Depcrate_validitystring {
() => {
// Module: crate::validity
// Provides: {"string"}
// Dependencies: {}
# [test] fn string () { assert ! (is_valid_string ("") . is_ok ()) ; assert ! (is_valid_string ("Hell\0") . is_err ()) ; assert ! (is_valid_string ("\u{ffff}") . is_ok ()) ; }
};
}
