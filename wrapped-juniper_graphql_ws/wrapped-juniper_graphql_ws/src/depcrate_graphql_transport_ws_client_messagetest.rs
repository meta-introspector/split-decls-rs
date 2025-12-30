// Generated macro for test (module)
macro_rules! Depcrate_graphql_transport_ws_client_messagetest {
() => {
// Module: crate::graphql_transport_ws::client_message
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use juniper :: { DefaultScalarValue , graphql_vars } ; use super :: * ; # [test] fn test_deserialization () { type ClientMessage = super :: ClientMessage < DefaultScalarValue > ; assert_eq ! (ClientMessage :: ConnectionInit { payload : graphql_vars ! { "foo" : "bar" } , } , serde_json :: from_str (r#"{"type": "connection_init", "payload": {"foo": "bar"}}"#) . unwrap () ,) ; assert_eq ! (ClientMessage :: ConnectionInit { payload : graphql_vars ! { } , } , serde_json :: from_str (r#"{"type": "connection_init"}"#) . unwrap () ,) ; assert_eq ! (ClientMessage :: Subscribe { id : "foo" . into () , payload : SubscribePayload { query : "query MyQuery { __typename }" . into () , variables : graphql_vars ! { "foo" : "bar" } , operation_name : Some ("MyQuery" . into ()) , extensions : Default :: default () , } , } , serde_json :: from_str (r#"{"type": "subscribe", "id": "foo", "payload": {
                "query": "query MyQuery { __typename }",
                "variables": {
                    "foo": "bar"
                },
                "operationName": "MyQuery"
            }}"#) . unwrap () ,) ; assert_eq ! (ClientMessage :: Subscribe { id : "foo" . into () , payload : SubscribePayload { query : "query MyQuery { __typename }" . into () , variables : graphql_vars ! { } , operation_name : None , extensions : Default :: default () , } , } , serde_json :: from_str (r#"{"type": "subscribe", "id": "foo", "payload": {
                "query": "query MyQuery { __typename }"
            }}"#) . unwrap () ,) ; assert_eq ! (ClientMessage :: Complete { id : "foo" . into () } , serde_json :: from_str (r#"{"type": "complete", "id": "foo"}"#) . unwrap () ,) ; } # [test] fn test_deserialization_of_null () -> serde_json :: Result < () > { let payload = r#"{"query":"query","variables":null}"# ; let payload : SubscribePayload < DefaultScalarValue > = serde_json :: from_str (payload) ? ; let expected = SubscribePayload { query : "query" . into () , variables : graphql_vars ! { } , operation_name : None , extensions : Default :: default () , } ; assert_eq ! (expected , payload) ; Ok (()) } }
};
}
