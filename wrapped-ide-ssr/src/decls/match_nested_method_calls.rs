macro_rules! match_nested_method_calls {
    () => {
        # [test] fn match_nested_method_calls () { assert_matches ("$a.z().z().z()" , "fn f() {h().i().j().z().z().z().d().e()}" , & ["h().i().j().z().z().z()"] ,) ; }
    };
}

match_nested_method_calls!();