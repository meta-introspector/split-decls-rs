// Generated macro for graphql_handler_sync (function)
macro_rules! Depcrategraphql_handler_sync {
() => {
// Module: crate
// Provides: {"graphql_handler_sync"}
// Dependencies: {}
# [doc = " Same as [`graphql_handler()`], but for [executing synchronously][1]."] # [doc = ""] # [doc = " [1]: GraphQLBatchRequest::execute_sync"] async fn graphql_handler_sync < Query , Mutation , Subscription , CtxT , S > (req : GraphQLBatchRequest < S > , schema : Arc < juniper :: RootNode < Query , Mutation , Subscription , S > > , context : CtxT ,) -> reply :: Response where Query : juniper :: GraphQLType < S , Context = CtxT > + Send + Sync + 'static , Query :: TypeInfo : Send + Sync , Mutation : juniper :: GraphQLType < S , Context = CtxT > + Send + Sync + 'static , Mutation :: TypeInfo : Send + Sync , Subscription : juniper :: GraphQLType < S , Context = CtxT > + Send + Sync + 'static , Subscription :: TypeInfo : Send + Sync , CtxT : Send + Sync + 'static , S : ScalarValue + Send + Sync + 'static , { task :: spawn_blocking (move | | req . execute_sync (& * schema , & context)) . await . map (| resp | JuniperResponse (resp) . into_response ()) . unwrap_or_else (| e | BlockingError (e) . into_response ()) }
};
}
