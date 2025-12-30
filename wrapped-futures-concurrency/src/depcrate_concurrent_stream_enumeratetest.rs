// Generated macro for test (module)
macro_rules! Depcrate_concurrent_stream_enumeratetest {
() => {
// Module: crate::concurrent_stream::enumerate
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: prelude :: * ; use futures_lite :: stream ; use futures_lite :: StreamExt ; use std :: num :: NonZeroUsize ; # [test] fn enumerate () { futures_lite :: future :: block_on (async { let mut n = 0 ; stream :: iter (std :: iter :: from_fn (| | { let v = n ; n += 1 ; Some (v) })) . take (5) . co () . limit (NonZeroUsize :: new (1)) . enumerate () . for_each (| (index , n) | async move { assert_eq ! (index , n) ; }) . await ; }) ; } }
};
}
