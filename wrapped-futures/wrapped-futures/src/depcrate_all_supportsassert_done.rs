// Generated macro for sassert_done (function)
macro_rules! Depcrate_all_supportsassert_done {
() => {
// Module: crate::all::support
// Provides: {"sassert_done"}
// Dependencies: {}
pub fn sassert_done < S : Stream > (s : & mut S) { match s . poll (& mut Task :: new ()) { Poll :: Ok (None) => { } Poll :: Ok (Some (_)) => panic ! ("stream had more elements") , Poll :: Err (_) => panic ! ("stream had an error") , Poll :: NotReady => panic ! ("stream wasn't ready") , } }
};
}
