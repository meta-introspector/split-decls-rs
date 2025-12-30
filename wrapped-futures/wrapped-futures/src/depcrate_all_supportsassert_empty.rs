// Generated macro for sassert_empty (function)
macro_rules! Depcrate_all_supportsassert_empty {
() => {
// Module: crate::all::support
// Provides: {"sassert_empty"}
// Dependencies: {}
pub fn sassert_empty < S : Stream > (s : & mut S) { match s . poll (& mut Task :: new ()) { Poll :: Ok (None) => panic ! ("stream is at its end") , Poll :: Ok (Some (_)) => panic ! ("stream had more elements") , Poll :: Err (_) => panic ! ("stream had an error") , Poll :: NotReady => { } } }
};
}
