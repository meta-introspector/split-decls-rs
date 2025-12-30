// Generated macro for test (module)
macro_rules! Depcrate_future_try_join_arraytest {
() => {
// Module: crate::future::try_join::array
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use core :: future ; # [test] fn all_ok () { futures_lite :: future :: block_on (async { let res : Result < _ , () > = [future :: ready (Ok ("hello")) , future :: ready (Ok ("world"))] . try_join () . await ; assert_eq ! (res . unwrap () , ["hello" , "world"]) ; }) } # [test] fn empty () { futures_lite :: future :: block_on (async { let data : [future :: Ready < Result < () , () > > ; 0] = [] ; let res = data . try_join () . await ; assert_eq ! (res . unwrap () , []) ; }) ; } # [test] fn one_err () { futures_lite :: future :: block_on (async { let res : Result < _ , _ > = [future :: ready (Ok ("hello")) , future :: ready (Err ("oh no"))] . try_join () . await ; assert_eq ! (res . unwrap_err () , "oh no") ; }) ; } }
};
}
