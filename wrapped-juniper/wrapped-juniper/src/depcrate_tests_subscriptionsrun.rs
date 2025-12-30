// Generated macro for run (function)
macro_rules! Depcrate_tests_subscriptionsrun {
() => {
// Module: crate::tests::subscriptions
// Provides: {"run"}
// Dependencies: {}
fn run < O > (f : impl Future < Output = O >) -> O { let rt = tokio :: runtime :: Runtime :: new () . unwrap () ; rt . block_on (f) }
};
}
