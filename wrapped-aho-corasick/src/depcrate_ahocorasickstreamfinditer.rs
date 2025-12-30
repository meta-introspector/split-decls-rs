// Generated macro for StreamFindIter (struct)
macro_rules! Depcrate_ahocorasickStreamFindIter {
() => {
// Module: crate::ahocorasick
// Provides: {"StreamFindIter"}
// Dependencies: {}
# [doc = " An iterator that reports Aho-Corasick matches in a stream."] # [doc = ""] # [doc = " This iterator yields elements of type `Result<Match, std::io::Error>`,"] # [doc = " where an error is reported if there was a problem reading from the"] # [doc = " underlying stream. The iterator terminates only when the underlying stream"] # [doc = " reaches `EOF`."] # [doc = ""] # [doc = " This iterator is constructed via the [`AhoCorasick::stream_find_iter`] and"] # [doc = " [`AhoCorasick::try_stream_find_iter`] methods."] # [doc = ""] # [doc = " The type variable `R` refers to the `io::Read` stream that is being read"] # [doc = " from."] # [doc = ""] # [doc = " The lifetime `'a` refers to the lifetime of the corresponding"] # [doc = " [`AhoCorasick`] searcher."] # [cfg (feature = "std")] # [derive (Debug)] pub struct StreamFindIter < 'a , R > (automaton :: StreamFindIter < 'a , Arc < dyn AcAutomaton > , R > ,) ;
};
}
