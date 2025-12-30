// Generated macro for impl_1358 (impl)
macro_rules! Depcrate_tests_subscriptionsimpl_1358 {
() => {
// Module: crate::tests::subscriptions
// Provides: {"impl_1358"}
// Dependencies: {}
# [graphql_subscription (context = MyContext)] impl MySubscription { async fn async_human () -> HumanStream { Box :: pin (stream :: once (async { Human { id : "stream id" . into () , name : "stream name" . into () , home_planet : "stream home planet" . into () , } })) } async fn error_human () -> Result < HumanStream , FieldError > { Err (FieldError :: new ("handler error" , graphql :: value ! ("more details") ,)) } async fn human_with_context (context : & MyContext) -> HumanStream { let context_val = context . 0 ; Box :: pin (stream :: once (async move { Human { id : context_val . to_string () , name : context_val . to_string () , home_planet : context_val . to_string () , } })) } async fn human_with_args (id : String , name : String) -> HumanStream { Box :: pin (stream :: once (async { Human { id , name , home_planet : "default home planet" . into () , } })) } }
};
}
