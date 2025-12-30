// Generated macro for tests (module)
macro_rules! Depcrate_stream_zip_tupletests {
() => {
// Module: crate::stream::zip::tuple
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use futures_lite :: future :: block_on ; use futures_lite :: prelude :: * ; use futures_lite :: stream ; use crate :: stream :: Zip ; # [test] fn zip_tuple_3 () { block_on (async { let a = stream :: repeat (1) . take (2) ; let b = stream :: repeat ("hello") . take (2) ; let c = stream :: repeat (("a" , "b")) . take (2) ; let mut s = Zip :: zip ((a , b , c)) ; assert_eq ! (s . next () . await , Some ((1 , "hello" , ("a" , "b")))) ; assert_eq ! (s . next () . await , Some ((1 , "hello" , ("a" , "b")))) ; assert_eq ! (s . next () . await , None) ; }) } }
};
}
