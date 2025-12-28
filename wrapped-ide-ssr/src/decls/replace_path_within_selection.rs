macro_rules! replace_path_within_selection {
    () => {
        # [test] fn replace_path_within_selection () { assert_ssr_transform ("foo ==>> bar" , r#"
        fn main() {
            let foo = 41;
            let bar = 42;
            do_stuff(foo);
            do_stuff(foo);$0
            do_stuff(foo);
            do_stuff(foo);$0
            do_stuff(foo);
        }"# , expect ! [[r#"
            fn main() {
                let foo = 41;
                let bar = 42;
                do_stuff(foo);
                do_stuff(foo);
                do_stuff(bar);
                do_stuff(bar);
                do_stuff(foo);
            }"#]] ,) ; }
    };
}

replace_path_within_selection!();