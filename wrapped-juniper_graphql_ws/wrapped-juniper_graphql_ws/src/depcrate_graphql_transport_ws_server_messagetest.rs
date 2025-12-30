// Generated macro for test (module)
macro_rules! Depcrate_graphql_transport_ws_server_messagetest {
() => {
// Module: crate::graphql_transport_ws::server_message
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use juniper :: { DefaultScalarValue , GraphQLError , graphql_value } ; use super :: * ; # [test] fn test_serialization () { type ServerMessage = super :: ServerMessage < DefaultScalarValue > ; assert_eq ! (serde_json :: to_string (& ServerMessage :: ConnectionAck) . unwrap () , r#"{"type":"connection_ack"}"# ,) ; assert_eq ! (serde_json :: to_string (& ServerMessage :: Pong) . unwrap () , r#"{"type":"pong"}"# ,) ; assert_eq ! (serde_json :: to_string (& ServerMessage :: Next { id : "foo" . into () , payload : NextPayload { data : graphql_value ! (null) , errors : vec ! [] , } , }) . unwrap () , r#"{"type":"next","id":"foo","payload":{"data":null}}"# ,) ; assert_eq ! (serde_json :: to_string (& ServerMessage :: Error { id : "foo" . into () , payload : GraphQLError :: UnknownOperationName . into () , }) . unwrap () , r#"{"type":"error","id":"foo","payload":[{"message":"Unknown operation"}]}"# ,) ; assert_eq ! (serde_json :: to_string (& ServerMessage :: Complete { id : "foo" . into () }) . unwrap () , r#"{"type":"complete","id":"foo"}"# ,) ; } }
};
}
