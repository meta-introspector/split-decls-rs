macro_rules! attribute_macro_doc_desugaring {
    () => {
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

attribute_macro_doc_desugaring!()