macro_rules! test_copy_expand_in_core {
    () => {
        # [test] fn test_copy_expand_in_core () { check (r#"
//- /lib.rs crate:core
#[rustc_builtin_macro]
macro derive {}
#[rustc_builtin_macro]
macro Copy {}
#[derive(Copy)]
struct Foo;
"# , expect ! [[r#"
#[rustc_builtin_macro]
macro derive {}
#[rustc_builtin_macro]
macro Copy {}
#[derive(Copy)]
struct Foo;

impl <> $crate::marker::Copy for Foo< > where {}"#]] ,) ; }
    };
}

test_copy_expand_in_core!()