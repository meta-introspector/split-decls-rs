// Generated macro for test (module)
macro_rules! Depcrate_future_race_ok_tupletest {
() => {
// Module: crate::future::race_ok::tuple
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use core :: future ; # [test] fn race_ok_1 () { futures_lite :: future :: block_on (async { let a = async { Ok :: < _ , () > ("world") } ; let res = (a ,) . race_ok () . await ; assert ! (matches ! (res , Ok ("world"))) ; }) ; } # [test] fn race_ok_2 () { futures_lite :: future :: block_on (async { let a = future :: pending () ; let b = async { Ok :: < _ , () > ("world") } ; let res = (a , b) . race_ok () . await ; assert ! (matches ! (res , Ok ("world"))) ; }) ; } # [test] fn race_ok_3 () { futures_lite :: future :: block_on (async { let a = future :: pending () ; let b = async { Ok :: < _ , () > ("hello") } ; let c = async { Ok :: < _ , () > ("world") } ; let result = (a , b , c) . race_ok () . await ; assert ! (matches ! (result , Ok ("hello") | Ok ("world"))) ; }) ; } # [test] fn race_ok_err () { futures_lite :: future :: block_on (async { let a = async { Err :: < () , _ > ("hello") } ; let b = async { Err :: < () , _ > ("world") } ; let errors = (a , b) . race_ok () . await . unwrap_err () ; assert_eq ! (errors [0] , "hello") ; assert_eq ! (errors [1] , "world") ; }) ; } # [test] fn race_ok_resume_after_completion () { use futures_lite :: future :: yield_now ; futures_lite :: future :: block_on (async { let ok = async { yield_now () . await ; yield_now () . await ; Ok :: < _ , () > (()) } ; let err = async { Err :: < () , _ > (()) } ; let res = (ok , err) . race_ok () . await ; assert_eq ! (res . ok () . unwrap () , ()) ; }) ; } }
};
}
