// Generated macro for impl_177 (impl)
macro_rules! Depcrate_schemaimpl_177 {
() => {
// Module: crate::schema
// Provides: {"impl_177"}
// Dependencies: {}
# [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] impl < Query , Mutation , Subscription > Executor for Schema < Query , Mutation , Subscription > where Query : ObjectType + 'static , Mutation : ObjectType + 'static , Subscription : SubscriptionType + 'static , { async fn execute (& self , request : Request) -> Response { Schema :: execute (self , request) . await } fn execute_stream (& self , request : Request , session_data : Option < Arc < Data > > ,) -> Pin < Box < dyn Stream < Item = Response > + Send + 'static > > { Schema :: execute_stream_with_session_data (& self , request , session_data . unwrap_or_default ()) } }
};
}
