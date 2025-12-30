// Generated macro for graphql_handler (function)
macro_rules! Depcrategraphql_handler {
() => {
// Module: crate
// Provides: {"graphql_handler"}
// Dependencies: {}
# [doc = " Actix Web GraphQL Handler for GET and POST requests"] pub async fn graphql_handler < Query , Mutation , Subscription , CtxT , S > (schema : & juniper :: RootNode < Query , Mutation , Subscription , S > , context : & CtxT , req : HttpRequest , payload : actix_web :: web :: Payload ,) -> Result < HttpResponse , Error > where Query : juniper :: GraphQLTypeAsync < S , Context = CtxT > , Query :: TypeInfo : Sync , Mutation : juniper :: GraphQLTypeAsync < S , Context = CtxT > , Mutation :: TypeInfo : Sync , Subscription : juniper :: GraphQLSubscriptionType < S , Context = CtxT > , Subscription :: TypeInfo : Sync , CtxT : Sync , S : ScalarValue + Send + Sync , { match * req . method () { Method :: POST => post_graphql_handler (schema , context , req , payload) . await , Method :: GET => get_graphql_handler (schema , context , req) . await , _ => Err (actix_web :: error :: UrlGenerationError :: ResourceNotFound . into ()) , } }
};
}
