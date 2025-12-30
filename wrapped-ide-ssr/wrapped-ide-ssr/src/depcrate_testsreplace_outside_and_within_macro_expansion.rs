// Generated macro for replace_outside_and_within_macro_expansion (function)
macro_rules! Depcrate_testsreplace_outside_and_within_macro_expansion {
() => {
// Module: crate::tests
// Provides: {"replace_outside_and_within_macro_expansion"}
// Dependencies: {}
# [test] fn replace_outside_and_within_macro_expansion () { assert_ssr_transform ("foo($a) ==>> bar($a)" , r#"
            fn foo() {} fn bar() {}
            macro_rules! macro1 {
                ($a:expr) => {$a}
            }
            fn f() {foo(foo(macro1!(foo(foo(42)))))}
            "# , expect ! [[r#"
            fn foo() {} fn bar() {}
            macro_rules! macro1 {
                ($a:expr) => {$a}
            }
            fn f() {bar(bar(macro1!(bar(bar(42)))))}
        "#]] ,) }
};
}
