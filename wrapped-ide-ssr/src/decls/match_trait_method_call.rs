macro_rules! match_trait_method_call {
    () => {
        # [test] fn match_trait_method_call () { let code = r#"
        pub(crate) struct Bar {}
        pub(crate) struct Bar2 {}
        pub(crate) trait Foo {
            fn foo(&self, _: i32) {}
        }
        impl Foo for Bar {}
        impl Foo for Bar2 {}
        fn main() {
            let v1 = Bar {};
            let v2 = Bar2 {};
            let v1_ref = &v1;
            let v2_ref = &v2;
            v1.foo(1);
            v2.foo(2);
            Bar::foo(&v1, 3);
            Bar2::foo(&v2, 4);
            v1_ref.foo(5);
            v2_ref.foo(6);
        }
        "# ; assert_matches ("Bar::foo($a, $b)" , code , & ["v1.foo(1)" , "Bar::foo(&v1, 3)" , "v1_ref.foo(5)"]) ; assert_matches ("Bar2::foo($a, $b)" , code , & ["v2.foo(2)" , "Bar2::foo(&v2, 4)" , "v2_ref.foo(6)"]) ; }
    };
}

match_trait_method_call!();