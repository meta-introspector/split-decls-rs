macro_rules! assist_order_if_expr {
    () => {
        # [test] fn assist_order_if_expr () { let (db , frange) = RootDatabase :: with_range (r#"
pub fn test_some_range(a: int) -> bool {
    if let 2..6 = $05$0 {
        true
    } else {
        false
    }
}
"# ,) ; let assists = assists (& db , & TEST_CONFIG , AssistResolveStrategy :: None , FileRange { file_id : frange . file_id . file_id (& db) , range : frange . range } ,) ; let expected = labels (& assists) ; expect ! [[r#"
        Extract into...
        Replace if let with match
    "#]] . assert_eq (& expected) ; }
    };
}

assist_order_if_expr!();