// Generated macro for floats (module)
macro_rules! Depcrate_executor_tests_variablesfloats {
() => {
// Module: crate::executor_tests::variables
// Provides: {"floats"}
// Dependencies: {}
mod floats { use super :: * ; # [tokio :: test] async fn float_values_should_work () { run_variable_query (r#"query q($var: Float!) { floatInput(value: $var) }"# , graphql :: vars ! { "var" : 10.0 } , | result | { assert_eq ! (result . get_field_value ("floatInput") , Some (& graphql :: value ! (r#"value: 10"#)) ,) ; } ,) . await ; } # [tokio :: test] async fn coercion_from_integers_should_work () { run_variable_query (r#"query q($var: Float!) { floatInput(value: $var) }"# , graphql :: vars ! { "var" : - 1 } , | result | { assert_eq ! (result . get_field_value ("floatInput") , Some (& graphql :: value ! (r#"value: -1"#)) ,) ; } ,) . await ; } # [tokio :: test] async fn does_not_coerce_from_string () { let schema = RootNode :: new (TestType , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; let query = r#"query q($var: Float!) { floatInput(value: $var) }"# ; let vars = graphql :: vars ! { "var" : "10" } ; let error = crate :: execute (query , None , & schema , & vars , & ()) . await . unwrap_err () ; assert_eq ! (error , RuleError :: new ("Variable \"$var\" got invalid value. \
                 Expected input scalar `Float`. Got: `\"10\"`. \
                 Details: Expected `Float`, found: \"10\"." , & [SourcePosition :: new (8 , 0 , 8)] ,) . into () ,) ; } }
};
}
