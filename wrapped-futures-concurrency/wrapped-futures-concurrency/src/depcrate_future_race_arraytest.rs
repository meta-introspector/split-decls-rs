// Generated macro for test (module)
macro_rules! Depcrate_future_race_arraytest {
() => {
// Module: crate::future::race::array
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use core :: future ; # [test] fn no_fairness () { futures_lite :: future :: block_on (async { let res = [future :: ready ("hello") , future :: ready ("world")] . race () . await ; assert ! (matches ! (res , "hello" | "world")) ; }) ; } }
};
}
