macro_rules! replace_function_call_with_placeholders {
    () => {
        # [test] fn replace_function_call_with_placeholders () { assert_ssr_transform ("foo($a, $b) ==>> bar($b, $a)" , "fn foo() {} fn bar() {} fn f1() {foo(5, 42)}" , expect ! [["fn foo() {} fn bar() {} fn f1() {bar(42, 5)}"]] ,) ; }
    };
}

replace_function_call_with_placeholders!()