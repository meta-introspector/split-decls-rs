// Generated macro for TEST_NAME_COUNTERS (static)
macro_rules! Depcrate_runtimeTEST_NAME_COUNTERS {
() => {
// Module: crate::runtime
// Provides: {"TEST_NAME_COUNTERS"}
// Dependencies: {}
static TEST_NAME_COUNTERS : Lazy < Mutex < BTreeMap < String , usize > > > = Lazy :: new (| | Mutex :: new (BTreeMap :: new ())) ;
};
}
