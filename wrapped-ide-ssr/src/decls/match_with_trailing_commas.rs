macro_rules! match_with_trailing_commas {
    () => {
        # [test] fn match_with_trailing_commas () { assert_matches ("foo($a, $b)" , "fn foo() {} fn f() {foo(1, 2,);}" , & ["foo(1, 2,)"]) ; assert_matches ("Foo{$a, $b}" , "struct Foo {} fn f() {Foo{1, 2,};}" , & ["Foo{1, 2,}"]) ; assert_matches ("foo($a, $b,)" , "fn foo() {} fn f() {foo(1, 2);}" , & ["foo(1, 2)"]) ; assert_matches ("Foo{$a, $b,}" , "struct Foo {} fn f() {Foo{1, 2};}" , & ["Foo{1, 2}"]) ; }
    };
}

match_with_trailing_commas!()