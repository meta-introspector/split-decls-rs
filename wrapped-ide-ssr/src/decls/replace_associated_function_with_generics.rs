macro_rules! replace_associated_function_with_generics {
    () => {
        # [test] fn replace_associated_function_with_generics () { assert_ssr_transform ("c::Foo::<$a>::new() ==>> d::Bar::<$a>::default()" , r#"
            mod c {
                pub(crate) struct Foo<T> {v: T}
                impl<T> Foo<T> { pub(crate) fn new() {} }
                fn f1() {
                    Foo::<i32>::new();
                }
            }
            mod d {
                pub(crate) struct Bar<T> {v: T}
                impl<T> Bar<T> { pub(crate) fn default() {} }
                fn f1() {
                    super::c::Foo::<i32>::new();
                }
            }
            "# , expect ! [[r#"
            mod c {
                pub(crate) struct Foo<T> {v: T}
                impl<T> Foo<T> { pub(crate) fn new() {} }
                fn f1() {
                    crate::d::Bar::<i32>::default();
                }
            }
            mod d {
                pub(crate) struct Bar<T> {v: T}
                impl<T> Bar<T> { pub(crate) fn default() {} }
                fn f1() {
                    Bar::<i32>::default();
                }
            }
            "#]] ,) ; }
    };
}

replace_associated_function_with_generics!();