// Generated macro for graphql_sync (function)
macro_rules! Depcrategraphql_sync {
() => {
// Module: crate
// Provides: {"graphql_sync"}
// Dependencies: {}
# [doc = " Executes synchronously  the provided GraphQL [`Request`] against the provided `schema` in the"] # [doc = " provided `context`, returning the encoded [`Response`]."] pub async fn graphql_sync < CtxT , QueryT , MutationT , SubscriptionT , S , B > (schema : Arc < RootNode < QueryT , MutationT , SubscriptionT , S > > , context : Arc < CtxT > , req : Request < B > ,) -> Response < String > where QueryT : GraphQLType < S , Context = CtxT > , QueryT :: TypeInfo : Sync , MutationT : GraphQLType < S , Context = CtxT > , MutationT :: TypeInfo : Sync , SubscriptionT : GraphQLType < S , Context = CtxT > , SubscriptionT :: TypeInfo : Sync , CtxT : Sync , S : ScalarValue + Send + Sync , B : Body < Error : Display > , { match parse_req (req) . await { Ok (req) => execute_request_sync (schema , context , req) . await , Err (resp) => resp , } }
};
}
