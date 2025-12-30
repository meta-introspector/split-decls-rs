// Generated macro for tests (module)
macro_rules! Depcrate_integrations_serdetests {
() => {
// Module: crate::integrations::serde
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use serde_json :: { from_str , to_string } ; use crate :: { DefaultScalarValue , FieldError , InputValue , graphql } ; use super :: { ExecutionError , GraphQLError } ; # [test] fn int () { assert_eq ! (from_str ::< InputValue > ("1235") . unwrap () , graphql :: input_value ! (1235) ,) ; } # [test] fn float () { assert_eq ! (from_str ::< InputValue > ("2.0") . unwrap () , graphql :: input_value ! (2.0) ,) ; assert_eq ! (from_str ::< InputValue > ("123567890123") . unwrap () , graphql :: input_value ! (123_567_890_123.0) ,) ; } # [test] fn errors () { assert_eq ! (to_string (& GraphQLError :: UnknownOperationName) . unwrap () , r#"[{"message":"Unknown operation"}]"# ,) ; } # [test] fn error_extensions () { assert_eq ! (to_string (& ExecutionError :: at_origin (FieldError ::< DefaultScalarValue >:: new ("foo error" , graphql :: value ! ({ "foo" : "bar" })) ,)) . unwrap () , r#"{"message":"foo error","locations":[{"line":1,"column":1}],"path":[],"extensions":{"foo":"bar"}}"# ,) ; } }
};
}
