// Generated macro for spec_input_value_fmt (module)
macro_rules! Depcrate_astspec_input_value_fmt {
() => {
// Module: crate::ast
// Provides: {"spec_input_value_fmt"}
// Dependencies: {}
# [cfg (test)] mod spec_input_value_fmt { use crate :: graphql ; use super :: InputValue ; # [test] fn correct () { let value : InputValue = graphql :: input_value ! (null) ; assert_eq ! (value . to_string () , "null") ; let value : InputValue = graphql :: input_value ! (123) ; assert_eq ! (value . to_string () , "123") ; let value : InputValue = graphql :: input_value ! (12.3) ; assert_eq ! (value . to_string () , "12.3") ; let value : InputValue = graphql :: input_value ! ("FOO") ; assert_eq ! (value . to_string () , "\"FOO\"") ; let value : InputValue = graphql :: input_value ! (true) ; assert_eq ! (value . to_string () , "true") ; let value : InputValue = graphql :: input_value ! (BAR) ; assert_eq ! (value . to_string () , "BAR") ; let value : InputValue = graphql :: input_value ! (@ baz) ; assert_eq ! (value . to_string () , "$baz") ; let value : InputValue = graphql :: input_value ! ([1 , 2]) ; assert_eq ! (value . to_string () , "[1, 2]") ; let value : InputValue = graphql :: input_value ! ({ "foo" : 1 , "bar" : 2 }) ; assert_eq ! (value . to_string () , "{foo: 1, bar: 2}") ; } }
};
}
