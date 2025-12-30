// Generated macro for test (module)
macro_rules! Depcrate_concurrent_stream_from_concurrent_streamtest {
() => {
// Module: crate::concurrent_stream::from_concurrent_stream
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: prelude :: * ; use futures_lite :: stream ; # [test] fn collect () { futures_lite :: future :: block_on (async { let v : Vec < _ > = stream :: repeat (1) . co () . take (5) . collect () . await ; assert_eq ! (v , & [1 , 1 , 1 , 1 , 1]) ; }) ; } # [test] fn collect_to_result_ok () { futures_lite :: future :: block_on (async { let v : Result < Vec < _ > , () > = stream :: repeat (Ok (1)) . co () . take (5) . collect () . await ; assert_eq ! (v , Ok (vec ! [1 , 1 , 1 , 1 , 1])) ; }) ; } # [test] fn collect_to_result_err () { futures_lite :: future :: block_on (async { let v : Result < Vec < _ > , _ > = stream :: repeat (Err :: < u8 , _ > (())) . co () . take (5) . collect () . await ; assert_eq ! (v , Err (())) ; }) ; } }
};
}
