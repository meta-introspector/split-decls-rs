// Generated macro for replace_associated_trait_default_function_call (function)
macro_rules! Depcrate_testsreplace_associated_trait_default_function_call {
() => {
// Module: crate::tests
// Provides: {"replace_associated_trait_default_function_call"}
// Dependencies: {}
# [test] fn replace_associated_trait_default_function_call () { cov_mark :: check ! (replace_associated_trait_default_function_call) ; assert_ssr_transform ("Bar2::foo() ==>> Bar2::foo2()" , r#"
            trait Foo { fn foo() {} }
            pub(crate) struct Bar {}
            impl Foo for Bar {}
            pub(crate) struct Bar2 {}
            impl Foo for Bar2 {}
            impl Bar2 { fn foo2() {} }
            fn main() {
                Bar::foo();
                Bar2::foo();
            }
        "# , expect ! [[r#"
            trait Foo { fn foo() {} }
            pub(crate) struct Bar {}
            impl Foo for Bar {}
            pub(crate) struct Bar2 {}
            impl Foo for Bar2 {}
            impl Bar2 { fn foo2() {} }
            fn main() {
                Bar::foo();
                Bar2::foo2();
            }
        "#]] ,) ; }
};
}
