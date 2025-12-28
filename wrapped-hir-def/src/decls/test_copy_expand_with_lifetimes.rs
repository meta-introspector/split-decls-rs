macro_rules! test_copy_expand_with_lifetimes {
    () => {
        # [test] fn test_copy_expand_with_lifetimes () { check (r#"
//- minicore: derive, copy
#[derive(Copy)]
struct Foo<A, B, 'a, 'b>;
"# , expect ! [[r#"
#[derive(Copy)]
struct Foo<A, B, 'a, 'b>;

impl <A: $crate::marker::Copy, B: $crate::marker::Copy, > $crate::marker::Copy for Foo<A, B, > where {}"#]] ,) ; }
    };
}

test_copy_expand_with_lifetimes!();