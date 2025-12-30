// Generated macro for IntoIter (struct)
macro_rules! Depcrate_stream_select_allIntoIter {
() => {
// Module: crate::stream::select_all
// Provides: {"IntoIter"}
// Dependencies: {}
# [doc = " Owned iterator over all streams in the unordered set."] # [derive (Debug)] pub struct IntoIter < St : Unpin > (futures_unordered :: IntoIter < StreamFuture < St > >) ;
};
}
