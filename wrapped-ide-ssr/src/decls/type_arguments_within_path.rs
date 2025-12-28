macro_rules! type_arguments_within_path {
    () => {
        # [test] fn type_arguments_within_path () { cov_mark :: check ! (type_arguments_within_path) ; let code = r#"
        mod foo {
            pub(crate) struct Bar<T> {t: T}
            impl<T> Bar<T> {
                pub(crate) fn baz() {}
            }
        }
        fn f1() {foo::Bar::<i32>::baz();}
        "# ; assert_no_match ("foo::Bar::<i64>::baz()" , code) ; assert_matches ("foo::Bar::<i32>::baz()" , code , & ["foo::Bar::<i32>::baz()"]) ; }
    };
}

type_arguments_within_path!();