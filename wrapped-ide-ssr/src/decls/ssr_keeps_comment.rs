macro_rules! ssr_keeps_comment {
    () => {
        # [test] fn ssr_keeps_comment () { assert_ssr_transform ("foo($x) ==>> bar($x)" , "fn foo() {} fn bar() {} fn main() { foo(5 /* using 5 */) }" , expect ! [["fn foo() {} fn bar() {} fn main() { bar(5)/* using 5 */ }"]] ,) }
    };
}

ssr_keeps_comment!();