// Generated macro for execute_request_sync (function)
macro_rules! Depcrateexecute_request_sync {
() => {
// Module: crate
// Provides: {"execute_request_sync"}
// Dependencies: {}
async fn execute_request_sync < CtxT , QueryT , MutationT , SubscriptionT , S > (schema : Arc < RootNode < QueryT , MutationT , SubscriptionT , S > > , context : Arc < CtxT > , request : GraphQLBatchRequest < S > ,) -> Response < String > where QueryT : GraphQLType < S , Context = CtxT > , QueryT :: TypeInfo : Sync , MutationT : GraphQLType < S , Context = CtxT > , MutationT :: TypeInfo : Sync , SubscriptionT : GraphQLType < S , Context = CtxT > , SubscriptionT :: TypeInfo : Sync , CtxT : Sync , S : ScalarValue + Send + Sync , { let res = request . execute_sync (& * schema , & context) ; let body = serde_json :: to_string_pretty (& res) . unwrap () ; let code = if res . is_ok () { StatusCode :: OK } else { StatusCode :: BAD_REQUEST } ; let mut resp = new_response (code) ; resp . headers_mut () . insert (header :: CONTENT_TYPE , HeaderValue :: from_static ("application/json") ,) ; * resp . body_mut () = body ; resp }
};
}
