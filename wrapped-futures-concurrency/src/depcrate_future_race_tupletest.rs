// Generated macro for test (module)
macro_rules! Depcrate_future_race_tupletest {
() => {
// Module: crate::future::race::tuple
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use core :: future ; # [test] fn race_1 () { futures_lite :: future :: block_on (async { let a = future :: ready ("world") ; assert_eq ! ((a ,) . race () . await , "world") ; }) ; } # [test] fn race_2 () { futures_lite :: future :: block_on (async { let a = future :: pending () ; let b = future :: ready ("world") ; assert_eq ! ((a , b) . race () . await , "world") ; }) ; } # [test] fn race_3 () { futures_lite :: future :: block_on (async { let a = future :: pending () ; let b = future :: ready ("hello") ; let c = future :: ready ("world") ; let result = (a , b , c) . race () . await ; assert ! (matches ! (result , "hello" | "world")) ; }) ; } }
};
}
