// Generated macro for test (module)
macro_rules! Depcrate_future_try_join_vectest {
() => {
// Module: crate::future::try_join::vec
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use alloc :: vec ; use core :: future ; # [test] fn all_ok () { futures_lite :: future :: block_on (async { let res : Result < _ , () > = vec ! [future :: ready (Ok ("hello")) , future :: ready (Ok ("world"))] . try_join () . await ; assert_eq ! (res . unwrap () , ["hello" , "world"]) ; }) } # [test] fn empty () { futures_lite :: future :: block_on (async { let data : Vec < future :: Ready < Result < () , () > > > = vec ! [] ; let res = data . try_join () . await ; assert_eq ! (res . unwrap () , vec ! []) ; }) ; } # [test] fn one_err () { futures_lite :: future :: block_on (async { let res : Result < _ , _ > = vec ! [future :: ready (Ok ("hello")) , future :: ready (Err ("oh no"))] . try_join () . await ; assert_eq ! (res . unwrap_err () , "oh no") ; }) ; } }
};
}
