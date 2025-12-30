// Generated macro for TEST_NAME_CLASH_DETECTION (static)
macro_rules! Depcrate_runtimeTEST_NAME_CLASH_DETECTION {
() => {
// Module: crate::runtime
// Provides: {"TEST_NAME_CLASH_DETECTION"}
// Dependencies: {}
static TEST_NAME_CLASH_DETECTION : Lazy < Mutex < BTreeMap < String , bool > > > = Lazy :: new (| | Mutex :: new (BTreeMap :: new ())) ;
};
}
