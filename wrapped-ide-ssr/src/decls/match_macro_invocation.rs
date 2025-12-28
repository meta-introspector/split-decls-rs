macro_rules! match_macro_invocation {
    () => {
        # [test] fn match_macro_invocation () { assert_matches ("foo!($a)" , "macro_rules! foo {() => {}} fn() {foo(foo!(foo()))}" , & ["foo!(foo())"] ,) ; assert_matches ("foo!(41, $a, 43)" , "macro_rules! foo {() => {}} fn() {foo!(41, 42, 43)}" , & ["foo!(41, 42, 43)"] ,) ; assert_no_match ("foo!(50, $a, 43)" , "macro_rules! foo {() => {}} fn() {foo!(41, 42, 43}") ; assert_no_match ("foo!(41, $a, 50)" , "macro_rules! foo {() => {}} fn() {foo!(41, 42, 43}") ; assert_matches ("foo!($a())" , "macro_rules! foo {() => {}} fn() {foo!(bar())}" , & ["foo!(bar())"] ,) ; }
    };
}

match_macro_invocation!()