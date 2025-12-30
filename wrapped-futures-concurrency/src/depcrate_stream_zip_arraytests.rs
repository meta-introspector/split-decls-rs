// Generated macro for tests (module)
macro_rules! Depcrate_stream_zip_arraytests {
() => {
// Module: crate::stream::zip::array
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: stream :: Zip ; use futures_lite :: future :: block_on ; use futures_lite :: prelude :: * ; use futures_lite :: stream ; # [test] fn zip_array_3 () { block_on (async { let a = stream :: repeat (1) . take (2) ; let b = stream :: repeat (2) . take (2) ; let c = stream :: repeat (3) . take (2) ; let mut s = Zip :: zip ([a , b , c]) ; assert_eq ! (s . next () . await , Some ([1 , 2 , 3])) ; assert_eq ! (s . next () . await , Some ([1 , 2 , 3])) ; assert_eq ! (s . next () . await , None) ; }) } }
};
}
