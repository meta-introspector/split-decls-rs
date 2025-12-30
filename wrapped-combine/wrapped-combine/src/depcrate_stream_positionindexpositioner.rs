// Generated macro for IndexPositioner (struct)
macro_rules! Depcrate_stream_positionIndexPositioner {
() => {
// Module: crate::stream::position
// Provides: {"IndexPositioner"}
// Dependencies: {}
# [doc = " The `IndexPositioner<Item, Range>` struct maintains the current index into the stream `Input`.  The"] # [doc = " initial index is index 0.  Each `Item` committed increments the index by 1; each `range` committed"] # [doc = " increments the position by `range.len()`."] # [derive (Clone , Debug , Default , PartialEq)] pub struct IndexPositioner (usize) ;
};
}
