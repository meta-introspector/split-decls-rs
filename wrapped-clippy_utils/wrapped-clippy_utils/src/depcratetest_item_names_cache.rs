// Generated macro for TEST_ITEM_NAMES_CACHE (static)
macro_rules! DepcrateTEST_ITEM_NAMES_CACHE {
() => {
// Module: crate
// Provides: {"TEST_ITEM_NAMES_CACHE"}
// Dependencies: {}
static TEST_ITEM_NAMES_CACHE : OnceLock < Mutex < FxHashMap < LocalModDefId , Vec < Symbol > > > > = OnceLock :: new () ;
};
}
