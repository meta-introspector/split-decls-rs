macro_rules! attribute_macro_syntax_completion_2 {
    () => {
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

attribute_macro_syntax_completion_2!()