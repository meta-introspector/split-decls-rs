// Generated macro for replace_within_macro_expansion (function)
macro_rules! Depcrate_testsreplace_within_macro_expansion {
() => {
// Module: crate::tests
// Provides: {"replace_within_macro_expansion"}
// Dependencies: {}
# [test] fn replace_within_macro_expansion () { assert_ssr_transform ("$a.foo() ==>> bar($a)" , r#"
            macro_rules! macro1 {
                ($a:expr) => {$a}
            }
            fn bar() {}
            fn f() {macro1!(5.x().foo().o2())}
            "# , expect ! [[r#"
            macro_rules! macro1 {
                ($a:expr) => {$a}
            }
            fn bar() {}
            fn f() {macro1!(bar(5.x()).o2())}
            "#]] ,) }
};
}
