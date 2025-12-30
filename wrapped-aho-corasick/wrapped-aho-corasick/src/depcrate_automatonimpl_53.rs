// Generated macro for impl_53 (impl)
macro_rules! Depcrate_automatonimpl_53 {
() => {
// Module: crate::automaton
// Provides: {"impl_53"}
// Dependencies: {}
# [cfg (feature = "std")] impl < 'a , A : Automaton , R : std :: io :: Read > Iterator for StreamFindIter < 'a , A , R > { type Item = std :: io :: Result < Match > ; fn next (& mut self) -> Option < std :: io :: Result < Match > > { loop { match self . it . next () { None => return None , Some (Err (err)) => return Some (Err (err)) , Some (Ok (StreamChunk :: NonMatch { .. })) => { } Some (Ok (StreamChunk :: Match { mat , .. })) => { return Some (Ok (mat)) ; } } } } }
};
}
