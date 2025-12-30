// Generated macro for CLIENT_PORTS (static)
macro_rules! Depcrate_tests_utilCLIENT_PORTS {
() => {
// Module: crate::tests::util
// Provides: {"CLIENT_PORTS"}
// Dependencies: {}
pub (crate) static CLIENT_PORTS : LazyLock < Mutex < RangeFrom < u16 > > > = LazyLock :: new (| | Mutex :: new (44433 ..)) ;
};
}
