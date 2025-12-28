macro_rules! ssr_keeps_nested_comment {
    () => {
        # [test] fn ssr_keeps_nested_comment () { assert_ssr_transform ("foo($x) ==>> bar($x)" , "fn foo() {} fn bar() {} fn main() { foo(other(5 /* using 5 */)) }" , expect ! [["fn foo() {} fn bar() {} fn main() { bar(other(5 /* using 5 */)) }"]] ,) }
    };
}

ssr_keeps_nested_comment!();