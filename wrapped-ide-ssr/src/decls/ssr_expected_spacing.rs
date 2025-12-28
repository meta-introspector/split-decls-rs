macro_rules! ssr_expected_spacing {
    () => {
        # [test] fn ssr_expected_spacing () { assert_ssr_transform ("foo($x) + bar() ==>> bar($x)" , "fn foo() {} fn bar() {} fn main() { foo(5) + bar() }" , expect ! [["fn foo() {} fn bar() {} fn main() { bar(5) }"]] ,) ; }
    };
}

ssr_expected_spacing!();