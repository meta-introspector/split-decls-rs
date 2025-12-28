macro_rules! replace_autoref_autoderef_capture {
    () => {
        # [test] fn replace_autoref_autoderef_capture () { cov_mark :: check ! (replace_autoref_autoderef_capture) ; let code = r#"
        struct Foo {}
        impl Foo {
            fn foo(&self) {}
            fn foo2(&self) {}
        }
        fn bar(_: &Foo) {}
        fn main() {
            let f = Foo {};
            let fr = &f;
            let fr2 = &fr;
            let fr3 = &fr2;
            f.foo();
            fr.foo();
            fr2.foo();
            fr3.foo();
        }
        "# ; assert_ssr_transform ("Foo::foo($a) ==>> bar($a)" , code , expect ! [[r#"
            struct Foo {}
            impl Foo {
                fn foo(&self) {}
                fn foo2(&self) {}
            }
            fn bar(_: &Foo) {}
            fn main() {
                let f = Foo {};
                let fr = &f;
                let fr2 = &fr;
                let fr3 = &fr2;
                bar(&f);
                bar(&*fr);
                bar(&**fr2);
                bar(&***fr3);
            }
        "#]] ,) ; assert_ssr_transform ("Foo::foo($a) ==>> $a.foo2()" , code , expect ! [[r#"
            struct Foo {}
            impl Foo {
                fn foo(&self) {}
                fn foo2(&self) {}
            }
            fn bar(_: &Foo) {}
            fn main() {
                let f = Foo {};
                let fr = &f;
                let fr2 = &fr;
                let fr3 = &fr2;
                f.foo2();
                fr.foo2();
                fr2.foo2();
                fr3.foo2();
            }
        "#]] ,) ; }
    };
}

replace_autoref_autoderef_capture!();