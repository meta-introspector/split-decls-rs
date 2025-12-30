// Generated macro for test (module)
macro_rules! Depcrate_graphql_ws_server_messagetest {
() => {
// Module: crate::graphql_ws::server_message
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use juniper :: { DefaultScalarValue , GraphQLError , graphql_value } ; use super :: * ; # [test] fn test_serialization () { type ServerMessage = super :: ServerMessage < DefaultScalarValue > ; assert_eq ! (serde_json :: to_string (& ServerMessage :: ConnectionError { payload : ConnectionErrorPayload { message : "foo" . into () , } , }) . unwrap () , r#"{"type":"connection_error","payload":{"message":"foo"}}"# ,) ; assert_eq ! (serde_json :: to_string (& ServerMessage :: ConnectionAck) . unwrap () , r#"{"type":"connection_ack"}"# ,) ; assert_eq ! (serde_json :: to_string (& ServerMessage :: Data { id : "foo" . into () , payload : DataPayload { data : graphql_value ! (null) , errors : vec ! [] , } , }) . unwrap () , r#"{"type":"data","id":"foo","payload":{"data":null}}"# ,) ; assert_eq ! (serde_json :: to_string (& ServerMessage :: Error { id : "foo" . into () , payload : GraphQLError :: UnknownOperationName . into () , }) . unwrap () , r#"{"type":"error","id":"foo","payload":[{"message":"Unknown operation"}]}"# ,) ; assert_eq ! (serde_json :: to_string (& ServerMessage :: Complete { id : "foo" . into () }) . unwrap () , r#"{"type":"complete","id":"foo"}"# ,) ; assert_eq ! (serde_json :: to_string (& ServerMessage :: ConnectionKeepAlive) . unwrap () , r#"{"type":"ka"}"# ,) ; } }
};
}
