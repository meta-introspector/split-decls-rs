// Generated macro for integers (module)
macro_rules! Depcrate_executor_tests_variablesintegers {
() => {
// Module: crate::executor_tests::variables
// Provides: {"integers"}
// Dependencies: {}
mod integers { use super :: * ; # [tokio :: test] async fn positive_and_negative_should_work () { run_variable_query (r#"query q($var: Int!) { integerInput(value: $var) }"# , graphql :: vars ! { "var" : 1 } , | result | { assert_eq ! (result . get_field_value ("integerInput") , Some (& graphql :: value ! (r#"value: 1"#)) ,) ; } ,) . await ; run_variable_query (r#"query q($var: Int!) { integerInput(value: $var) }"# , graphql :: vars ! { "var" : - 1 } , | result | { assert_eq ! (result . get_field_value ("integerInput") , Some (& graphql :: value ! (r#"value: -1"#)) ,) ; } ,) . await ; } # [tokio :: test] async fn does_not_coerce_from_float () { let schema = RootNode :: new (TestType , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; let query = r#"query q($var: Int!) { integerInput(value: $var) }"# ; let vars = graphql :: vars ! { "var" : 10.0 } ; let error = crate :: execute (query , None , & schema , & vars , & ()) . await . unwrap_err () ; assert_eq ! (error , RuleError :: new ("Variable \"$var\" got invalid value. Expected input scalar `Int`. \
                 Got: `10`. Details: Expected `Int`, found: 10." , & [SourcePosition :: new (8 , 0 , 8)] ,) . into () ,) ; } # [tokio :: test] async fn does_not_coerce_from_string () { let schema = RootNode :: new (TestType , EmptyMutation :: < () > :: new () , EmptySubscription :: < () > :: new () ,) ; let query = r#"query q($var: Int!) { integerInput(value: $var) }"# ; let vars = graphql :: vars ! { "var" : "10" } ; let error = crate :: execute (query , None , & schema , & vars , & ()) . await . unwrap_err () ; assert_eq ! (error , RuleError :: new ("Variable \"$var\" got invalid value. \
                 Expected input scalar `Int`. Got: `\"10\"`. \
                 Details: Expected `Int`, found: \"10\"." , & [SourcePosition :: new (8 , 0 , 8)] ,) . into () ,) ; } }
};
}
