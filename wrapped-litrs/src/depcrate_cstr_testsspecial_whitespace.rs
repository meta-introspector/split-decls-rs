// Generated macro for special_whitespace (function)
macro_rules! Depcrate_cstr_testsspecial_whitespace {
() => {
// Module: crate::cstr::tests
// Provides: {"special_whitespace"}
// Dependencies: {}
# [test] fn special_whitespace () { let strings = ["\n" , "\t" , "foo\tbar" , "baz\n"] ; for & s in & strings { let input = format ! (r#"c"{}""# , s) ; let input_raw = format ! (r#"cr"{}""# , s) ; let value = CString :: new (s) . unwrap () ; for (input , num_hashes) in vec ! [(input , None) , (input_raw , Some (0))] { let expected = CStringLit { raw : & * input , value : value . clone () , num_hashes , start_suffix : input . len () , } ; assert_parse_ok_eq (& input , CStringLit :: parse (& * input) , expected . clone () , "CStringLit::parse") ; assert_parse_ok_eq (& input , Literal :: parse (& * input) , Literal :: CString (expected) , "Literal::parse") ; assert_eq ! (CStringLit :: parse (&* input) . unwrap () . value () , value . as_c_str ()) ; assert_eq ! (CStringLit :: parse (&* input) . unwrap () . into_value () , value) ; } } }
};
}
