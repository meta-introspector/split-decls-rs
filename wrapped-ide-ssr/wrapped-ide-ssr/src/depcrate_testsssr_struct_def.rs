// Generated macro for ssr_struct_def (function)
macro_rules! Depcrate_testsssr_struct_def {
() => {
// Module: crate::tests
// Provides: {"ssr_struct_def"}
// Dependencies: {}
# [test] fn ssr_struct_def () { assert_ssr_transform ("struct Foo { $f: $t } ==>> struct Foo($t);" , r#"struct Foo { field: i32 }"# , expect ! [[r#"struct Foo(i32);"#]] ,) }
};
}
