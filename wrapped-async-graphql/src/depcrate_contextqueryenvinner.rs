// Generated macro for QueryEnvInner (struct)
macro_rules! Depcrate_contextQueryEnvInner {
() => {
// Module: crate::context
// Provides: {"QueryEnvInner"}
// Dependencies: {}
# [doc (hidden)] pub struct QueryEnvInner { pub extensions : Extensions , pub variables : Variables , pub operation_name : Option < String > , pub operation : Positioned < OperationDefinition > , pub fragments : HashMap < Name , Positioned < FragmentDefinition > > , pub uploads : Vec < UploadValue > , pub session_data : Arc < Data > , pub query_data : Arc < Data > , pub http_headers : Mutex < http :: HeaderMap > , pub introspection_mode : IntrospectionMode , pub errors : Mutex < Vec < ServerError > > , }
};
}
