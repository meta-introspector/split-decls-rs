macro_rules! attribute_macro_syntax_completion_1 {
    () => {
        # [test] fn attribute_macro_syntax_completion_1 () { check (r#"
//- proc_macros: identity_when_valid
#[proc_macros::identity_when_valid]
fn foo() { bar.baz(); blub }
"# , expect ! [[r#"
#[proc_macros::identity_when_valid]
fn foo() { bar.baz(); blub }

fn foo() {
    bar.baz();
    blub
}"#]] ,) ; }
    };
}

attribute_macro_syntax_completion_1!();