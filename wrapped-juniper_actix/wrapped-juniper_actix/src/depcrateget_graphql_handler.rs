// Generated macro for get_graphql_handler (function)
macro_rules! Depcrateget_graphql_handler {
() => {
// Module: crate
// Provides: {"get_graphql_handler"}
// Dependencies: {}
# [doc = " Actix GraphQL Handler for GET requests"] pub async fn get_graphql_handler < Query , Mutation , Subscription , CtxT , S > (schema : & juniper :: RootNode < Query , Mutation , Subscription , S > , context : & CtxT , req : HttpRequest ,) -> Result < HttpResponse , Error > where Query : juniper :: GraphQLTypeAsync < S , Context = CtxT > , Query :: TypeInfo : Sync , Mutation : juniper :: GraphQLTypeAsync < S , Context = CtxT > , Mutation :: TypeInfo : Sync , Subscription : juniper :: GraphQLSubscriptionType < S , Context = CtxT > , Subscription :: TypeInfo : Sync , CtxT : Sync , S : ScalarValue + Send + Sync , { let get_req = web :: Query :: < GetGraphQLRequest > :: from_query (req . query_string ()) ? ; let req = GraphQLRequest :: from (get_req . into_inner ()) ; let gql_response = req . execute (schema , context) . await ; let body_response = serde_json :: to_string (& gql_response) ? ; let mut response = match gql_response . is_ok () { true => HttpResponse :: Ok () , false => HttpResponse :: BadRequest () , } ; Ok (response . content_type ("application/json") . body (body_response)) }
};
}
