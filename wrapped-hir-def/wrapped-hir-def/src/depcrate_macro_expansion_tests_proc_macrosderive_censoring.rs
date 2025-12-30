// Generated macro for derive_censoring (function)
macro_rules! Depcrate_macro_expansion_tests_proc_macrosderive_censoring {
() => {
// Module: crate::macro_expansion_tests::proc_macros
// Provides: {"derive_censoring"}
// Dependencies: {}
# [test] fn derive_censoring () { cov_mark :: check ! (derive_censoring) ; check (r#"
//- proc_macros: derive_identity
//- minicore:derive
#[attr1]
#[derive(Foo)]
#[derive(proc_macros::DeriveIdentity)]
#[derive(Bar)]
#[attr2]
struct S;
"# , expect ! [[r#"
#[attr1]
#[derive(Foo)]
#[derive(proc_macros::DeriveIdentity)]
#[derive(Bar)]
#[attr2]
struct S;

#[attr1]
#[derive(Bar)]
#[attr2] struct S;"#]] ,) ; }
};
}
