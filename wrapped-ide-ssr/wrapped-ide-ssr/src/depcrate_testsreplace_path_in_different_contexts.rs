// Generated macro for replace_path_in_different_contexts (function)
macro_rules! Depcrate_testsreplace_path_in_different_contexts {
() => {
// Module: crate::tests
// Provides: {"replace_path_in_different_contexts"}
// Dependencies: {}
# [test] fn replace_path_in_different_contexts () { assert_ssr_transform ("c::foo() ==>> c::bar()" , r#"
            mod a {
                pub(crate) mod b {$0
                    pub(crate) mod c {
                        pub(crate) fn foo() {}
                        pub(crate) fn bar() {}
                        fn f1() { foo() }
                    }
                    fn f2() { c::foo() }
                }
                fn f3() { b::c::foo() }
            }
            use a::b::c::foo;
            fn f4() { foo() }
            "# , expect ! [[r#"
            mod a {
                pub(crate) mod b {
                    pub(crate) mod c {
                        pub(crate) fn foo() {}
                        pub(crate) fn bar() {}
                        fn f1() { bar() }
                    }
                    fn f2() { c::bar() }
                }
                fn f3() { b::c::bar() }
            }
            use a::b::c::foo;
            fn f4() { a::b::c::bar() }
            "#]] ,) ; }
};
}
