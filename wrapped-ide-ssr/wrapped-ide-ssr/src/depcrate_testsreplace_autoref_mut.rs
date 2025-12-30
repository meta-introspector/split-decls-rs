// Generated macro for replace_autoref_mut (function)
macro_rules! Depcrate_testsreplace_autoref_mut {
() => {
// Module: crate::tests
// Provides: {"replace_autoref_mut"}
// Dependencies: {}
# [test] fn replace_autoref_mut () { let code = r#"
        struct Foo {}
        impl Foo {
            fn foo(&mut self) {}
        }
        fn bar(_: &mut Foo) {}
        fn main() {
            let mut f = Foo {};
            f.foo();
            let fr = &mut f;
            fr.foo();
        }
        "# ; assert_ssr_transform ("Foo::foo($a) ==>> bar($a)" , code , expect ! [[r#"
            struct Foo {}
            impl Foo {
                fn foo(&mut self) {}
            }
            fn bar(_: &mut Foo) {}
            fn main() {
                let mut f = Foo {};
                bar(&mut f);
                let fr = &mut f;
                bar(&mut *fr);
            }
        "#]] ,) ; }
};
}
