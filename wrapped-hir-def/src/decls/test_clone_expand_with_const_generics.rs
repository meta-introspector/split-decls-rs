macro_rules! test_clone_expand_with_const_generics {
    () => {
        # [test] fn test_clone_expand_with_const_generics () { check (r#"
//- minicore: derive, clone
#[derive(Clone)]
struct Foo<const X: usize, T>(u32);
"# , expect ! [[r#"
#[derive(Clone)]
struct Foo<const X: usize, T>(u32);

impl <const X: usize, T: $crate::clone::Clone, > $crate::clone::Clone for Foo<X, T, > where {
    fn clone(&self ) -> Self {
        match self {
            Foo(f0, )=>Foo(f0.clone(), ),
        }
    }
}"#]] ,) ; }
    };
}

test_clone_expand_with_const_generics!();