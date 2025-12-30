// Generated macro for replace_path_within_selection (function)
macro_rules! Depcrate_testsreplace_path_within_selection {
() => {
// Module: crate::tests
// Provides: {"replace_path_within_selection"}
// Dependencies: {}
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
