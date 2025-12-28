macro_rules! match_nested_method_calls_with_macro_call {
    () => {
        # [test] fn match_nested_method_calls_with_macro_call () { assert_matches ("$a.z().z().z()" , r#"
            macro_rules! m1 { ($a:expr) => {$a}; }
            fn f() {m1!(h().i().j().z().z().z().d().e())}"# , & ["h().i().j().z().z().z()"] ,) ; }
    };
}

match_nested_method_calls_with_macro_call!()