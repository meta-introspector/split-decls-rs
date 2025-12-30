// Generated macro for test (module)
macro_rules! Depcrate_future_race_vectest {
() => {
// Module: crate::future::race::vec
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use alloc :: vec ; use core :: future ; # [test] fn no_fairness () { futures_lite :: future :: block_on (async { let res = vec ! [future :: ready ("hello") , future :: ready ("world")] . race () . await ; assert ! (matches ! (res , "hello" | "world")) ; }) ; } }
};
}
