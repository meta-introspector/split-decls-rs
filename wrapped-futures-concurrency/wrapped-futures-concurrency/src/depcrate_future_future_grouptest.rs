// Generated macro for test (module)
macro_rules! Depcrate_future_future_grouptest {
() => {
// Module: crate::future::future_group
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: FutureGroup ; use core :: future ; use futures_lite :: prelude :: * ; # [test] fn smoke () { futures_lite :: future :: block_on (async { let mut group = FutureGroup :: new () ; group . insert (future :: ready (2)) ; group . insert (future :: ready (4)) ; let mut out = 0 ; while let Some (num) = group . next () . await { out += num ; } assert_eq ! (out , 6) ; assert_eq ! (group . len () , 0) ; assert ! (group . is_empty ()) ; }) ; } # [test] fn capacity_grow_on_insert () { futures_lite :: future :: block_on (async { let mut group = FutureGroup :: new () ; let cap = group . capacity () ; group . insert (future :: ready (1)) ; assert ! (group . capacity () > cap) ; }) ; } }
};
}
