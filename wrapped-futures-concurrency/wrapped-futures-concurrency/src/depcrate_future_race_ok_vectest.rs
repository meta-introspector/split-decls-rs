// Generated macro for test (module)
macro_rules! Depcrate_future_race_ok_vectest {
() => {
// Module: crate::future::race_ok::vec
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use alloc :: vec ; use core :: future ; # [test] fn all_ok () { futures_lite :: future :: block_on (async { let res : Result < & str , AggregateError < () > > = vec ! [future :: ready (Ok ("hello")) , future :: ready (Ok ("world"))] . race_ok () . await ; assert ! (res . is_ok ()) ; }) } # [test] fn one_err () { futures_lite :: future :: block_on (async { let res : Result < & str , AggregateError < _ > > = vec ! [future :: ready (Ok ("hello")) , future :: ready (Err ("oh no"))] . race_ok () . await ; assert_eq ! (res . unwrap () , "hello") ; }) ; } # [test] fn all_err () { futures_lite :: future :: block_on (async { let res : Result < & str , AggregateError < _ > > = vec ! [future :: ready (Err ("oops")) , future :: ready (Err ("oh no"))] . race_ok () . await ; let errs = res . unwrap_err () ; assert_eq ! (errs [0] , "oops") ; assert_eq ! (errs [1] , "oh no") ; }) ; } }
};
}
