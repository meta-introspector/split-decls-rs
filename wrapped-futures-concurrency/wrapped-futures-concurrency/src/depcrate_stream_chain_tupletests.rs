// Generated macro for tests (module)
macro_rules! Depcrate_stream_chain_tupletests {
() => {
// Module: crate::stream::chain::tuple
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use futures_lite :: future :: block_on ; use futures_lite :: prelude :: * ; use futures_lite :: stream ; # [test] fn chain_3 () { block_on (async { let a = stream :: once (1) ; let b = stream :: once (2) ; let c = stream :: once (3) ; let mut s = (a , b , c) . chain () ; assert_eq ! (s . next () . await , Some (1)) ; assert_eq ! (s . next () . await , Some (2)) ; assert_eq ! (s . next () . await , Some (3)) ; assert_eq ! (s . next () . await , None) ; }) } }
};
}
