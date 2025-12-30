// Generated macro for parse_test_case (function)
macro_rules! Depcrate_testparse_test_case {
() => {
// Module: crate::test
// Provides: {"parse_test_case"}
// Dependencies: {}
fn parse_test_case (current_section : & mut String , lines : & mut dyn Iterator < Item = & str > ,) -> Option < TestCase > { let mut attributes = Vec :: new () ; let mut is_first_line = true ; loop { let line = lines . next () ; # [cfg (feature = "test_logging")] { if let Some (text) = & line { println ! ("Line: {}" , text) ; } } match line { None if is_first_line => { return None ; } None => { return Some (TestCase { attributes }) ; } Some ("") => { if ! is_first_line { return Some (TestCase { attributes }) ; } } Some (line) if line . starts_with ('#') => () , Some (line) if line . starts_with ('[') => { assert ! (is_first_line) ; assert ! (line . ends_with (']')) ; current_section . truncate (0) ; current_section . push_str (line) ; let _ : Option < char > = current_section . pop () ; let _ : char = current_section . remove (0) ; } Some (line) => { is_first_line = false ; let parts : Vec < & str > = line . splitn (2 , " = ") . collect () ; assert_eq ! (parts . len () , 2 , "Syntax error: Expected Key = Value.") ; let key = parts [0] . trim () ; let value = parts [1] . trim () ; assert_ne ! (value . len () , 0) ; attributes . push ((String :: from (key) , String :: from (value) , false)) ; } } } }
};
}
