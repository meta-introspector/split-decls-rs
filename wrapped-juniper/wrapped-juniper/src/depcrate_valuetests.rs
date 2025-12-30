// Generated macro for tests (module)
macro_rules! Depcrate_valuetests {
() => {
// Module: crate::value
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: graphql ; use super :: Value ; # [test] fn display_null () { let s : Value = graphql :: value ! (null) ; assert_eq ! (s . to_string () , "null") ; } # [test] fn display_int () { let s : Value = graphql :: value ! (123) ; assert_eq ! (s . to_string () , "123") ; } # [test] fn display_float () { let s : Value = graphql :: value ! (123.456) ; assert_eq ! (s . to_string () , "123.456") ; } # [test] fn display_string () { let s : Value = graphql :: value ! ("foo") ; assert_eq ! (s . to_string () , "\"foo\"") ; } # [test] fn display_bool () { let s : Value = graphql :: value ! (false) ; assert_eq ! (s . to_string () , "false") ; let s : Value = graphql :: value ! (true) ; assert_eq ! (s . to_string () , "true") ; } # [test] fn display_list () { let s : Value = graphql :: value ! ([1 , null , "foo"]) ; assert_eq ! (s . to_string () , "[1, null, \"foo\"]") ; } # [test] fn display_list_one_element () { let s : Value = graphql :: value ! ([1]) ; assert_eq ! (s . to_string () , "[1]") ; } # [test] fn display_list_empty () { let s : Value = graphql :: value ! ([]) ; assert_eq ! (s . to_string () , "[]") ; } # [test] fn display_object () { let s : Value = graphql :: value ! ({ "int" : 1 , "null" : null , "string" : "foo" , }) ; assert_eq ! (s . to_string () , r#"{"int": 1, "null": null, "string": "foo"}"# ,) ; } # [test] fn display_object_one_field () { let s : Value = graphql :: value ! ({ "int" : 1 , }) ; assert_eq ! (s . to_string () , r#"{"int": 1}"#) ; } # [test] fn display_object_empty () { let s : Value = graphql :: value ! ({ }) ; assert_eq ! (s . to_string () , r#"{}"#) ; } }
};
}
