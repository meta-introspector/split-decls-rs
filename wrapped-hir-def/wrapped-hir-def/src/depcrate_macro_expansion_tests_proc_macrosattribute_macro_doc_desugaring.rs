// Generated macro for attribute_macro_doc_desugaring (function)
macro_rules! Depcrate_macro_expansion_tests_proc_macrosattribute_macro_doc_desugaring {
() => {
// Module: crate::macro_expansion_tests::proc_macros
// Provides: {"attribute_macro_doc_desugaring"}
// Dependencies: {}
# [test] fn attribute_macro_doc_desugaring () { check (r#"
//- proc_macros: identity
#[proc_macros::identity]
/// doc string \n with newline
/**
     MultiLines Doc
     MultiLines Doc
*/
#[doc = "doc attr"]
struct S;
"# , expect ! [[r##"
#[proc_macros::identity]
/// doc string \n with newline
/**
     MultiLines Doc
     MultiLines Doc
*/
#[doc = "doc attr"]
struct S;

#[doc = " doc string \\n with newline"]
#[doc = "\n     MultiLines Doc\n     MultiLines Doc\n"]
#[doc = "doc attr"] struct S;"##]] ,) ; }
};
}
