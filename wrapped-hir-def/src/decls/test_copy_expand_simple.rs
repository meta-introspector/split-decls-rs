macro_rules! test_copy_expand_simple {
    () => {
        # [test] fn test_copy_expand_simple () { check (r#"
//- minicore: derive, copy
#[derive(Copy)]
struct Foo;
"# , expect ! [[r#"
#[derive(Copy)]
struct Foo;

impl <> $crate::marker::Copy for Foo< > where {}"#]] ,) ; }
    };
}

test_copy_expand_simple!();