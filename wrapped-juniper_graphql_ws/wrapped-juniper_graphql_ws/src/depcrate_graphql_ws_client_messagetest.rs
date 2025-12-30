// Generated macro for test (module)
macro_rules! Depcrate_graphql_ws_client_messagetest {
() => {
// Module: crate::graphql_ws::client_message
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use juniper :: { DefaultScalarValue , graphql_vars } ; use super :: * ; # [test] fn test_deserialization () { type ClientMessage = super :: ClientMessage < DefaultScalarValue > ; assert_eq ! (ClientMessage :: ConnectionInit { payload : graphql_vars ! { "foo" : "bar" } , } , serde_json :: from_str (r#"{"type": "connection_init", "payload": {"foo": "bar"}}"#) . unwrap () ,) ; assert_eq ! (ClientMessage :: ConnectionInit { payload : graphql_vars ! { } , } , serde_json :: from_str (r#"{"type": "connection_init"}"#) . unwrap () ,) ; assert_eq ! (ClientMessage :: Start { id : "foo" . into () , payload : StartPayload { query : "query MyQuery { __typename }" . into () , variables : graphql_vars ! { "foo" : "bar" } , operation_name : Some ("MyQuery" . into ()) , } , } , serde_json :: from_str (r#"{"type": "start", "id": "foo", "payload": {
                "query": "query MyQuery { __typename }",
                "variables": {
                    "foo": "bar"
                },
                "operationName": "MyQuery"
            }}"#) . unwrap () ,) ; assert_eq ! (ClientMessage :: Start { id : "foo" . into () , payload : StartPayload { query : "query MyQuery { __typename }" . into () , variables : graphql_vars ! { } , operation_name : None , } , } , serde_json :: from_str (r#"{"type": "start", "id": "foo", "payload": {
                "query": "query MyQuery { __typename }"
            }}"#) . unwrap () ,) ; assert_eq ! (ClientMessage :: Stop { id : "foo" . into () } , serde_json :: from_str (r#"{"type": "stop", "id": "foo"}"#) . unwrap () ,) ; assert_eq ! (ClientMessage :: ConnectionTerminate , serde_json :: from_str (r#"{"type": "connection_terminate"}"#) . unwrap () ,) ; } # [test] fn test_deserialization_of_null () -> serde_json :: Result < () > { let payload = r#"{"query":"query","variables":null}"# ; let payload : StartPayload < DefaultScalarValue > = serde_json :: from_str (payload) ? ; let expected = StartPayload { query : "query" . into () , variables : graphql_vars ! { } , operation_name : None , } ; assert_eq ! (expected , payload) ; Ok (()) } }
};
}
