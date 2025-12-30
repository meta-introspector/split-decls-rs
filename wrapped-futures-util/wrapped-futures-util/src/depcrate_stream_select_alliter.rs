// Generated macro for Iter (struct)
macro_rules! Depcrate_stream_select_allIter {
() => {
// Module: crate::stream::select_all
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " Immutable iterator over all streams in the unordered set."] # [derive (Debug)] pub struct Iter < 'a , St : Unpin > (futures_unordered :: Iter < 'a , StreamFuture < St > >) ;
};
}
