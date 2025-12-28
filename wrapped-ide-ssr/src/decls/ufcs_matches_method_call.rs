macro_rules! ufcs_matches_method_call {
    () => {
        # [test] fn ufcs_matches_method_call () { let code = r#"
    struct Foo {}
    impl Foo {
        fn new(_: i32) -> Foo { Foo {} }
        fn do_stuff(&self, _: i32) {}
    }
    struct Bar {}
    impl Bar {
        fn new(_: i32) -> Bar { Bar {} }
        fn do_stuff(&self, v: i32) {}
    }
    fn main() {
        let b = Bar {};
        let f = Foo {};
        b.do_stuff(1);
        f.do_stuff(2);
        Foo::new(4).do_stuff(3);
        // Too many / too few args - should never match
        f.do_stuff(2, 10);
        f.do_stuff();
    }
    "# ; assert_matches ("Foo::do_stuff($a, $b)" , code , & ["f.do_stuff(2)" , "Foo::new(4).do_stuff(3)"]) ; assert_matches ("Foo::do_stuff($a, 2)" , code , & ["f.do_stuff(2)"]) ; assert_matches ("Foo::do_stuff(Foo::new(4), $b)" , code , & ["Foo::new(4).do_stuff(3)"]) ; assert_ssr_transform ("Foo::do_stuff(Foo::new($a), $b) ==>> Bar::new($b).do_stuff($a)" , code , expect ! [[r#"
            struct Foo {}
            impl Foo {
                fn new(_: i32) -> Foo { Foo {} }
                fn do_stuff(&self, _: i32) {}
            }
            struct Bar {}
            impl Bar {
                fn new(_: i32) -> Bar { Bar {} }
                fn do_stuff(&self, v: i32) {}
            }
            fn main() {
                let b = Bar {};
                let f = Foo {};
                b.do_stuff(1);
                f.do_stuff(2);
                Bar::new(3).do_stuff(4);
                // Too many / too few args - should never match
                f.do_stuff(2, 10);
                f.do_stuff();
            }
        "#]] ,) ; }
    };
}

ufcs_matches_method_call!()