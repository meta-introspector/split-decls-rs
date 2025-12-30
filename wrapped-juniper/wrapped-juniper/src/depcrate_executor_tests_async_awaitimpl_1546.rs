// Generated macro for impl_1546 (impl)
macro_rules! Depcrate_executor_tests_async_awaitimpl_1546 {
() => {
// Module: crate::executor_tests::async_await
// Provides: {"impl_1546"}
// Dependencies: {}
# [graphql_object] impl User { async fn id (& self) -> i32 { self . id } async fn name (& self) -> & str { & self . name } async fn friends (& self) -> Vec < User > { (0 .. 10) . map (| index | User { id : index , name : format ! ("user{index}") , kind : UserKind :: User , }) . collect () } async fn kind (& self) -> & UserKind { & self . kind } async fn delayed () -> bool { tokio :: time :: sleep (std :: time :: Duration :: from_millis (100)) . await ; true } }
};
}
