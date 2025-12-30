// Generated macro for graphql (function)
macro_rules! Depcrategraphql {
() => {
// Module: crate
// Provides: {"graphql"}
// Dependencies: {}
# [doc = " Executes the provided GraphQL [`Request`] against the provided `schema` in the provided"] # [doc = " `context`, returning the encoded [`Response`]."] pub async fn graphql < CtxT , QueryT , MutationT , SubscriptionT , S , B > (schema : Arc < RootNode < QueryT , MutationT , SubscriptionT , S > > , context : Arc < CtxT > , req : Request < B > ,) -> Response < String > where QueryT : GraphQLTypeAsync < S , Context = CtxT > , QueryT :: TypeInfo : Sync , MutationT : GraphQLTypeAsync < S , Context = CtxT > , MutationT :: TypeInfo : Sync , SubscriptionT : GraphQLSubscriptionType < S , Context = CtxT > , SubscriptionT :: TypeInfo : Sync , CtxT : Sync , S : ScalarValue + Send + Sync , B : Body < Error : Display > , { match parse_req (req) . await { Ok (req) => execute_request (schema , context , req) . await , Err (resp) => resp , } }
};
}
