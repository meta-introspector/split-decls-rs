// Generated macro for RawIterRange (struct)
macro_rules! Depcrate_rawRawIterRange {
() => {
// Module: crate::raw
// Provides: {"RawIterRange"}
// Dependencies: {}
# [doc = " Iterator over a sub-range of a table. Unlike `RawIter` this iterator does"] # [doc = " not track an item count."] pub (crate) struct RawIterRange < T > { current_group : BitMaskIter , data : Bucket < T > , next_ctrl : * const u8 , end : * const u8 , }
};
}
