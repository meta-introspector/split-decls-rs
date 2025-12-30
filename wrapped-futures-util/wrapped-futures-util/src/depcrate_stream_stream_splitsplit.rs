// Generated macro for split (function)
macro_rules! Depcrate_stream_stream_splitsplit {
() => {
// Module: crate::stream::stream::split
// Provides: {"split"}
// Dependencies: {}
pub (super) fn split < S : Stream + Sink < Item > , Item > (s : S) -> (SplitSink < S , Item > , SplitStream < S >) { let (a , b) = BiLock :: new (s) ; let read = SplitStream (a) ; let write = SplitSink (b) ; (write , read) }
};
}
