// Generated macro for attribute_macro_attr_censoring (function)
macro_rules! Depcrate_macro_expansion_tests_proc_macrosattribute_macro_attr_censoring {
() => {
// Module: crate::macro_expansion_tests::proc_macros
// Provides: {"attribute_macro_attr_censoring"}
// Dependencies: {}
# [test] fn attribute_macro_attr_censoring () { cov_mark :: check ! (attribute_macro_attr_censoring) ; check (r#"
//- proc_macros: identity
#[attr1] #[proc_macros::identity] #[attr2]
struct S;
"# , expect ! [[r#"
#[attr1] #[proc_macros::identity] #[attr2]
struct S;

#[attr1]
#[attr2] struct S;"#]] ,) ; }
};
}
