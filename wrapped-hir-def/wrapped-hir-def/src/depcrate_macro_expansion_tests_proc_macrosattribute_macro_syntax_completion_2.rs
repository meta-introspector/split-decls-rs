// Generated macro for attribute_macro_syntax_completion_2 (function)
macro_rules! Depcrate_macro_expansion_tests_proc_macrosattribute_macro_syntax_completion_2 {
() => {
// Module: crate::macro_expansion_tests::proc_macros
// Provides: {"attribute_macro_syntax_completion_2"}
// Dependencies: {}
# [test] fn attribute_macro_syntax_completion_2 () { check (r#"
//- proc_macros: identity_when_valid
#[proc_macros::identity_when_valid]
fn foo() { bar.; blub }
"# , expect ! [[r#"
#[proc_macros::identity_when_valid]
fn foo() { bar.; blub }

fn foo() {
    bar. ;
    blub
}"#]] ,) ; }
};
}
