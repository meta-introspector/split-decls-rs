// Generated macro for post_graphql_handler (function)
macro_rules! Depcratepost_graphql_handler {
() => {
// Module: crate
// Provides: {"post_graphql_handler"}
// Dependencies: {}
# [doc = " Actix GraphQL Handler for POST requests"] pub async fn post_graphql_handler < Query , Mutation , Subscription , CtxT , S > (schema : & juniper :: RootNode < Query , Mutation , Subscription , S > , context : & CtxT , req : HttpRequest , payload : actix_web :: web :: Payload ,) -> Result < HttpResponse , Error > where Query : juniper :: GraphQLTypeAsync < S , Context = CtxT > , Query :: TypeInfo : Sync , Mutation : juniper :: GraphQLTypeAsync < S , Context = CtxT > , Mutation :: TypeInfo : Sync , Subscription : juniper :: GraphQLSubscriptionType < S , Context = CtxT > , Subscription :: TypeInfo : Sync , CtxT : Sync , S : ScalarValue + Send + Sync , { let req = match req . content_type () { "application/json" => { let body = String :: from_request (& req , & mut payload . into_inner ()) . await ? ; serde_json :: from_str :: < GraphQLBatchRequest < S > > (& body) . map_err (JsonPayloadError :: Deserialize) } "application/graphql" => { let body = String :: from_request (& req , & mut payload . into_inner ()) . await ? ; Ok (GraphQLBatchRequest :: Single (GraphQLRequest :: new (body , None , None ,))) } _ => Err (JsonPayloadError :: ContentType) , } ? ; let gql_batch_response = req . execute (schema , context) . await ; let gql_response = serde_json :: to_string (& gql_batch_response) ? ; let mut response = match gql_batch_response . is_ok () { true => HttpResponse :: Ok () , false => HttpResponse :: BadRequest () , } ; Ok (response . content_type ("application/json") . body (gql_response)) }
};
}
