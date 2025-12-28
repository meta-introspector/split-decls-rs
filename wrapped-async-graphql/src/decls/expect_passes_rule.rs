macro_rules! expect_passes_rule {
    () => {
        macro_rules ! expect_passes_rule { ($ factory : expr , $ query_source : literal $ (,) ?) => { let doc = crate :: parser :: parse_query ($ query_source) . expect ("Parse error") ; crate :: validation :: test_harness :: expect_passes_rule_ (& doc , $ factory) ; } ; }
    };
}

expect_passes_rule!();