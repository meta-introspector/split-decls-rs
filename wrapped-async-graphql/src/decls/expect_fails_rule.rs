macro_rules! expect_fails_rule {
    () => {
        macro_rules ! expect_fails_rule { ($ factory : expr , $ query_source : literal $ (,) ?) => { let doc = crate :: parser :: parse_query ($ query_source) . expect ("Parse error") ; crate :: validation :: test_harness :: expect_fails_rule_ (& doc , $ factory) ; } ; }
    };
}

expect_fails_rule!();