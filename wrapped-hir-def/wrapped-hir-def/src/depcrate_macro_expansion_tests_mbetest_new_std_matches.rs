// Generated macro for test_new_std_matches (function)
macro_rules! Depcrate_macro_expansion_tests_mbetest_new_std_matches {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"test_new_std_matches"}
// Dependencies: {}
# [test] fn test_new_std_matches () { check (r#"
macro_rules! matches {
    ($expression:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {
        match $expression {
            $pattern $(if $guard)? => true,
            _ => false
        }
    };
}
fn main() {
    matches!(0, 0 | 1 if true);
}
 "# , expect ! [[r#"
macro_rules! matches {
    ($expression:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {
        match $expression {
            $pattern $(if $guard)? => true,
            _ => false
        }
    };
}
fn main() {
    match 0 {
        0|1 if true =>true , _=>false
    };
}
 "#]] ,) ; }
};
}
