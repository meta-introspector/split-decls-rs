// Generated macro for preserves_whitespace_within_macro_expansion (function)
macro_rules! Depcrate_testspreserves_whitespace_within_macro_expansion {
() => {
// Module: crate::tests
// Provides: {"preserves_whitespace_within_macro_expansion"}
// Dependencies: {}
# [test] fn preserves_whitespace_within_macro_expansion () { assert_ssr_transform ("$a + $b ==>> $b - $a" , r#"
            macro_rules! macro1 {
                ($a:expr) => {$a}
            }
            fn f() {macro1!(1   *   2 + 3 + 4)}
            "# , expect ! [[r#"
            macro_rules! macro1 {
                ($a:expr) => {$a}
            }
            fn f() {macro1!(4 - (3 - 1   *   2))}
            "#]] ,) }
};
}
