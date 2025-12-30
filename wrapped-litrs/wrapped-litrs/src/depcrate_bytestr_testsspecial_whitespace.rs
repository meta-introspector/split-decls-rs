// Generated macro for special_whitespace (function)
macro_rules! Depcrate_bytestr_testsspecial_whitespace {
() => {
// Module: crate::bytestr::tests
// Provides: {"special_whitespace"}
// Dependencies: {}
# [test] fn special_whitespace () { let strings = ["\n" , "\t" , "foo\tbar" , "baz\n"] ; for & s in & strings { let input = format ! (r#"b"{}""# , s) ; let input_raw = format ! (r#"br"{}""# , s) ; for (input , num_hashes) in vec ! [(input , None) , (input_raw , Some (0))] { let expected = ByteStringLit { raw : & * input , value : None , num_hashes , start_suffix : input . len () , } ; assert_parse_ok_eq (& input , ByteStringLit :: parse (& * input) , expected . clone () , "ByteStringLit::parse" ,) ; assert_parse_ok_eq (& input , Literal :: parse (& * input) , Literal :: ByteString (expected) , "Literal::parse" ,) ; assert_eq ! (ByteStringLit :: parse (&* input) . unwrap () . value () , s . as_bytes ()) ; assert_eq ! (ByteStringLit :: parse (&* input) . unwrap () . into_value () , s . as_bytes ()) ; } } }
};
}
