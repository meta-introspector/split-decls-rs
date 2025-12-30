// Generated macro for impl_1295 (impl)
macro_rules! Depcrate_tests_fixtures_starwars_schemaimpl_1295 {
() => {
// Module: crate::tests::fixtures::starwars::schema
// Provides: {"impl_1295"}
// Dependencies: {}
# [graphql_subscription (context = Database)] # [doc = " Super basic subscription fixture"] impl Subscription { async fn async_human (context : & Database) -> HumanStream { let human = context . get_human ("1000") . unwrap () . clone () ; Box :: pin (futures :: stream :: once (futures :: future :: ready (human))) } }
};
}
