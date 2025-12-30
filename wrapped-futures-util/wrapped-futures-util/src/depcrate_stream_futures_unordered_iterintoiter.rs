// Generated macro for IntoIter (struct)
macro_rules! Depcrate_stream_futures_unordered_iterIntoIter {
() => {
// Module: crate::stream::futures_unordered::iter
// Provides: {"IntoIter"}
// Dependencies: {}
# [doc = " Owned iterator over all futures in the unordered set."] # [derive (Debug)] pub struct IntoIter < Fut : Unpin > { pub (super) len : usize , pub (super) inner : FuturesUnordered < Fut > , }
};
}
