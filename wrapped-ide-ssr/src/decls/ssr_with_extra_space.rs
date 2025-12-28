macro_rules! ssr_with_extra_space {
    () => {
        # [test] fn ssr_with_extra_space () { assert_ssr_transform ("foo($x  ) +    bar() ==>> bar($x)" , "fn foo() {} fn bar() {} fn main() { foo(  5 )  +bar(   ) }" , expect ! [["fn foo() {} fn bar() {} fn main() { bar(5) }"]] ,) ; }
    };
}

ssr_with_extra_space!()