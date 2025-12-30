// Generated macro for test (module)
macro_rules! Depcrate_stream_stream_grouptest {
() => {
// Module: crate::stream::stream_group
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: StreamGroup ; use futures_lite :: { prelude :: * , stream } ; # [test] fn smoke () { futures_lite :: future :: block_on (async { let mut group = StreamGroup :: new () ; group . insert (stream :: once (2)) ; group . insert (stream :: once (4)) ; let mut out = 0 ; while let Some (num) = group . next () . await { out += num ; } assert_eq ! (out , 6) ; assert_eq ! (group . len () , 0) ; assert ! (group . is_empty ()) ; }) ; } # [test] fn capacity_grow_on_insert () { futures_lite :: future :: block_on (async { let mut group = StreamGroup :: new () ; let cap = group . capacity () ; group . insert (stream :: once (1)) ; assert ! (group . capacity () > cap) ; }) ; } }
};
}
