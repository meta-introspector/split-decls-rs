// Generated macro for test (module)
macro_rules! Depcrate_collections_vectest {
() => {
// Module: crate::collections::vec
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: prelude :: * ; # [test] fn collect () { futures_lite :: future :: block_on (async { let v : Vec < _ > = vec ! [1 , 2 , 3 , 4 , 5] . into_co_stream () . collect () . await ; assert_eq ! (v , & [1 , 2 , 3 , 4 , 5]) ; }) ; } }
};
}
