macro_rules! test_copy_expand_with_type_params {
    () => {
        # [test] fn test_copy_expand_with_type_params () { check (r#"
//- minicore: derive, copy
#[derive(Copy)]
struct Foo<A, B>;
"# , expect ! [[r#"
#[derive(Copy)]
struct Foo<A, B>;

impl <A: $crate::marker::Copy, B: $crate::marker::Copy, > $crate::marker::Copy for Foo<A, B, > where {}"#]] ,) ; }
    };
}

test_copy_expand_with_type_params!()