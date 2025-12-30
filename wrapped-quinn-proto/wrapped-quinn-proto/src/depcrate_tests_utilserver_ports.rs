// Generated macro for SERVER_PORTS (static)
macro_rules! Depcrate_tests_utilSERVER_PORTS {
() => {
// Module: crate::tests::util
// Provides: {"SERVER_PORTS"}
// Dependencies: {}
static SERVER_PORTS : LazyLock < Mutex < RangeFrom < u16 > > > = LazyLock :: new (| | Mutex :: new (4433 ..)) ;
};
}
