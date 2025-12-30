// Generated macro for IterMut (struct)
macro_rules! Depcrate_stream_select_allIterMut {
() => {
// Module: crate::stream::select_all
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " Mutable iterator over all streams in the unordered set."] # [derive (Debug)] pub struct IterMut < 'a , St : Unpin > (futures_unordered :: IterMut < 'a , StreamFuture < St > >) ;
};
}
