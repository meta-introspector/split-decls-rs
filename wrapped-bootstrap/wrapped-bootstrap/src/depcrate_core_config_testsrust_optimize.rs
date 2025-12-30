// Generated macro for rust_optimize (function)
macro_rules! Depcrate_core_config_testsrust_optimize {
() => {
// Module: crate::core::config::tests
// Provides: {"rust_optimize"}
// Dependencies: {}
# [test] fn rust_optimize () { assert ! (parse ("") . rust_optimize . is_release ()) ; assert ! (! parse ("rust.optimize = false") . rust_optimize . is_release ()) ; assert ! (parse ("rust.optimize = true") . rust_optimize . is_release ()) ; assert ! (! parse ("rust.optimize = 0") . rust_optimize . is_release ()) ; assert ! (parse ("rust.optimize = 1") . rust_optimize . is_release ()) ; assert ! (parse ("rust.optimize = \"s\"") . rust_optimize . is_release ()) ; assert_eq ! (parse ("rust.optimize = 1") . rust_optimize . get_opt_level () , Some ("1" . to_string ())) ; assert_eq ! (parse ("rust.optimize = \"s\"") . rust_optimize . get_opt_level () , Some ("s" . to_string ())) ; }
};
}
