// Generated macro for sassert_next (function)
macro_rules! Depcrate_all_supportsassert_next {
() => {
// Module: crate::all::support
// Provides: {"sassert_next"}
// Dependencies: {}
pub fn sassert_next < S : Stream > (s : & mut S , item : S :: Item) where S :: Item : Eq + fmt :: Debug { match s . poll (& mut Task :: new ()) { Poll :: Ok (None) => panic ! ("stream is at its end") , Poll :: Ok (Some (e)) => assert_eq ! (e , item) , Poll :: Err (_) => panic ! ("stream had an error") , Poll :: NotReady => panic ! ("stream wasn't ready") , } }
};
}
