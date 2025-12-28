macro_rules! deps {
    () => {
        TestCtxt!();
    };
}

macro_rules! mk_tests_slice {
    () => {
        deps!();
        # [doc = " Creates a slice containing every test like so:"] # [doc = " &[&test1, &test2]"] fn mk_tests_slice (cx : & TestCtxt < '_ > , sp : Span) -> Box < ast :: Expr > { debug ! ("building test vector from {} tests" , cx . test_cases . len ()) ; let ecx = & cx . ext_cx ; let mut tests = cx . test_cases . clone () ; tests . sort_by (| a , b | a . name . as_str () . cmp (b . name . as_str ())) ; ecx . expr_array_ref (sp , tests . iter () . map (| test | { ecx . expr_addr_of (test . span , ecx . expr_path (ecx . path (test . span , vec ! [test . ident]))) }) . collect () ,) }
    };
}

mk_tests_slice!();