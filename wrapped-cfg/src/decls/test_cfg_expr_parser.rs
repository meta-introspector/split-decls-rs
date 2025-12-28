macro_rules! deps {
    () => {
        CfgExpr!();
        CfgAtom!();
    };
}

macro_rules! test_cfg_expr_parser {
    () => {
        deps!();
        # [test] fn test_cfg_expr_parser () { assert_parse_result ("#![cfg(foo)]" , CfgAtom :: Flag (Symbol :: intern ("foo")) . into ()) ; assert_parse_result ("#![cfg(foo,)]" , CfgAtom :: Flag (Symbol :: intern ("foo")) . into ()) ; assert_parse_result ("#![cfg(not(foo))]" , CfgExpr :: Not (Box :: new (CfgAtom :: Flag (Symbol :: intern ("foo")) . into ())) ,) ; assert_parse_result ("#![cfg(foo(bar))]" , CfgExpr :: Invalid) ; assert_parse_result (r#"#![cfg(foo, bar = "baz")]"# , CfgAtom :: Flag (Symbol :: intern ("foo")) . into () ,) ; assert_parse_result (r#"#![cfg(all(foo, bar = "baz"))]"# , CfgExpr :: All (vec ! [CfgAtom :: Flag (Symbol :: intern ("foo")) . into () , CfgAtom :: KeyValue { key : Symbol :: intern ("bar") , value : Symbol :: intern ("baz") } . into () ,] . into_boxed_slice () ,) ,) ; assert_parse_result (r#"#![cfg(any(not(), all(), , bar = "baz",))]"# , CfgExpr :: Any (vec ! [CfgExpr :: Not (Box :: new (CfgExpr :: Invalid)) , CfgExpr :: All (Box :: new ([])) , CfgExpr :: Invalid , CfgAtom :: KeyValue { key : Symbol :: intern ("bar") , value : Symbol :: intern ("baz") } . into () ,] . into_boxed_slice () ,) ,) ; }
    };
}

test_cfg_expr_parser!()