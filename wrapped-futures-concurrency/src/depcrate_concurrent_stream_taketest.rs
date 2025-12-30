// Generated macro for test (module)
macro_rules! Depcrate_concurrent_stream_taketest {
() => {
// Module: crate::concurrent_stream::take
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: prelude :: * ; use futures_lite :: stream ; # [test] fn enumerate () { futures_lite :: future :: block_on (async { let mut n = 0 ; stream :: iter (std :: iter :: from_fn (| | { let v = n ; n += 1 ; Some (v) })) . co () . take (5) . for_each (| n | async move { assert ! (n < 5) }) . await ; }) ; } }
};
}
