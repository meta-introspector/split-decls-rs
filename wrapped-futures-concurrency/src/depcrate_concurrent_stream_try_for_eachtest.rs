// Generated macro for test (module)
macro_rules! Depcrate_concurrent_stream_try_for_eachtest {
() => {
// Module: crate::concurrent_stream::try_for_each
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use crate :: prelude :: * ; use futures_lite :: stream ; use std :: io ; # [test] fn concurrency_one () { futures_lite :: future :: block_on (async { let count = Arc :: new (AtomicUsize :: new (0)) ; stream :: repeat (1) . take (2) . co () . limit (NonZeroUsize :: new (1)) . try_for_each (| n | { let count = count . clone () ; async move { count . fetch_add (n , Ordering :: Relaxed) ; std :: io :: Result :: Ok (()) } }) . await . unwrap () ; assert_eq ! (count . load (Ordering :: Relaxed) , 2) ; }) ; } # [test] fn concurrency_three () { futures_lite :: future :: block_on (async { let count = Arc :: new (AtomicUsize :: new (0)) ; stream :: repeat (1) . take (10) . co () . limit (NonZeroUsize :: new (3)) . try_for_each (| n | { let count = count . clone () ; async move { count . fetch_add (n , Ordering :: Relaxed) ; std :: io :: Result :: Ok (()) } }) . await . unwrap () ; assert_eq ! (count . load (Ordering :: Relaxed) , 10) ; }) ; } # [test] fn short_circuits () { futures_lite :: future :: block_on (async { let count = Arc :: new (AtomicUsize :: new (0)) ; let output = stream :: repeat (10) . take (2) . co () . limit (NonZeroUsize :: new (1)) . try_for_each (| n | { let count = count . clone () ; async move { count . fetch_add (n , Ordering :: SeqCst) ; std :: io :: Result :: Err (io :: ErrorKind :: Other . into ()) } }) . await ; assert ! (output . is_err ()) ; }) ; } }
};
}
