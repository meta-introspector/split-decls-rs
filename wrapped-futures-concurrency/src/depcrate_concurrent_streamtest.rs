// Generated macro for test (module)
macro_rules! Depcrate_concurrent_streamtest {
() => {
// Module: crate::concurrent_stream
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use crate :: prelude :: * ; use futures_lite :: prelude :: * ; use futures_lite :: stream ; # [test] fn drain () { futures_lite :: future :: block_on (async { stream :: repeat (1) . take (5) . co () . map (| x | async move { println ! ("{x:?}") ; }) . for_each (| _ | async { }) . await ; }) ; } # [test] fn for_each () { futures_lite :: future :: block_on (async { let s = stream :: repeat (1) . take (2) ; s . co () . limit (NonZeroUsize :: new (3)) . for_each (| x | async move { println ! ("{x:?}") ; }) . await ; }) ; } }
};
}
