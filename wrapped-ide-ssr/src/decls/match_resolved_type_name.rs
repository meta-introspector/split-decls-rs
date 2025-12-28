macro_rules! match_resolved_type_name {
    () => {
        # [test] fn match_resolved_type_name () { let code = r#"
        mod m1 {
            pub(crate) mod m2 {
                pub(crate) trait Foo<T> {}
            }
        }
        mod m3 {
            trait Foo<T> {}
            fn f1(f: Option<&dyn Foo<bool>>) {}
        }
        mod m4 {
            use crate::m1::m2::Foo;
            fn f1(f: Option<&dyn Foo<i32>>) {}
        }
        "# ; assert_matches ("m1::m2::Foo<$t>" , code , & ["Foo<i32>"]) ; }
    };
}

match_resolved_type_name!();