// Generated macro for test_concat_bytes_expand (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_fn_macrotest_concat_bytes_expand {
() => {
// Module: crate::macro_expansion_tests::builtin_fn_macro
// Provides: {"test_concat_bytes_expand"}
// Dependencies: {}
# [test] fn test_concat_bytes_expand () { check (r##"
#[rustc_builtin_macro]
macro_rules! concat_bytes {}

fn main() { concat_bytes!(b'A', b"BC\"", [68, b'E', 70], br#"G""#,b'\0'); }
"## , expect ! [[r#"
#[rustc_builtin_macro]
macro_rules! concat_bytes {}

fn main() { b"ABC\"DEFG\"\x00"; }
"#]] ,) ; }
};
}
