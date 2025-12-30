// Generated macro for replace_associated_function_call (function)
macro_rules! Depcrate_testsreplace_associated_function_call {
() => {
// Module: crate::tests
// Provides: {"replace_associated_function_call"}
// Dependencies: {}
# [test] fn replace_associated_function_call () { assert_ssr_transform ("Foo::new() ==>> Bar::new()" , r#"
            struct Foo {}
            impl Foo { fn new() {} }
            struct Bar {}
            impl Bar { fn new() {} }
            fn f1() {Foo::new();}
            "# , expect ! [[r#"
            struct Foo {}
            impl Foo { fn new() {} }
            struct Bar {}
            impl Bar { fn new() {} }
            fn f1() {Bar::new();}
        "#]] ,) ; }
};
}
