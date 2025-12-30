// Generated macro for test (module)
macro_rules! Depcrate_concurrent_stream_for_eachtest {
() => {
// Module: crate::concurrent_stream::for_each
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use crate :: prelude :: * ; use futures_lite :: stream ; # [test] fn concurrency_one () { futures_lite :: future :: block_on (async { let count = Arc :: new (AtomicUsize :: new (0)) ; stream :: repeat (1) . take (2) . co () . limit (NonZeroUsize :: new (1)) . for_each (| n | { let count = count . clone () ; async move { count . fetch_add (n , Ordering :: Relaxed) ; } }) . await ; assert_eq ! (count . load (Ordering :: Relaxed) , 2) ; }) ; } # [test] fn concurrency_three () { futures_lite :: future :: block_on (async { let count = Arc :: new (AtomicUsize :: new (0)) ; stream :: repeat (1) . take (10) . co () . limit (NonZeroUsize :: new (3)) . for_each (| n | { let count = count . clone () ; async move { count . fetch_add (n , Ordering :: Relaxed) ; } }) . await ; assert_eq ! (count . load (Ordering :: Relaxed) , 10) ; }) ; } }
};
}
