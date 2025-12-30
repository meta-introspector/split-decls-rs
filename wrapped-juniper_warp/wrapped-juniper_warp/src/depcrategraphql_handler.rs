// Generated macro for graphql_handler (function)
macro_rules! Depcrategraphql_handler {
() => {
// Module: crate
// Provides: {"graphql_handler"}
// Dependencies: {}
# [doc = " Executes the provided [`GraphQLBatchRequest`] against the provided `schema` in the provided"] # [doc = " `context`."] async fn graphql_handler < Query , Mutation , Subscription , CtxT , S > (req : GraphQLBatchRequest < S > , schema : Arc < juniper :: RootNode < Query , Mutation , Subscription , S > > , context : CtxT ,) -> reply :: Response where Query : juniper :: GraphQLTypeAsync < S , Context = CtxT > + Send + 'static , Query :: TypeInfo : Send + Sync , Mutation : juniper :: GraphQLTypeAsync < S , Context = CtxT > + Send + 'static , Mutation :: TypeInfo : Send + Sync , Subscription : juniper :: GraphQLSubscriptionType < S , Context = CtxT > + Send + 'static , Subscription :: TypeInfo : Send + Sync , CtxT : Send + Sync + 'static , S : ScalarValue + Send + Sync + 'static , { let resp = req . execute (& * schema , & context) . await ; JuniperResponse (resp) . into_response () }
};
}
