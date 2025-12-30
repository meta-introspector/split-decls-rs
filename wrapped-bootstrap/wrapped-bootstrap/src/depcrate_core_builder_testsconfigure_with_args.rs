// Generated macro for configure_with_args (function)
macro_rules! Depcrate_core_builder_testsconfigure_with_args {
() => {
// Module: crate::core::builder::tests
// Provides: {"configure_with_args"}
// Dependencies: {}
fn configure_with_args (cmd : & [& str] , host : & [& str] , target : & [& str]) -> Config { TestCtx :: new () . config (cmd [0]) . args (& cmd [1 ..]) . hosts (host) . targets (target) . args (& ["--build" , TEST_TRIPLE_1]) . create_config () }
};
}
