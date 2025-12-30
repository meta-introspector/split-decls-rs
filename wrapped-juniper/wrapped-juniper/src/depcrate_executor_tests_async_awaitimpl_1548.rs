// Generated macro for impl_1548 (impl)
macro_rules! Depcrate_executor_tests_async_awaitimpl_1548 {
() => {
// Module: crate::executor_tests::async_await
// Provides: {"impl_1548"}
// Dependencies: {}
# [graphql_object] impl Query { fn field_sync (& self) -> & 'static str { "field_sync" } async fn field_async_plain () -> String { "field_async_plain" . into () } fn user (id : String) -> User { User { id : 1 , name : id , kind : UserKind :: User , } } async fn delayed () -> bool { tokio :: time :: sleep (std :: time :: Duration :: from_millis (100)) . await ; true } }
};
}
