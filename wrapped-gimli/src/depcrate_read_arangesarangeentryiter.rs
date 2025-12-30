// Generated macro for ArangeEntryIter (struct)
macro_rules! Depcrate_read_arangesArangeEntryIter {
() => {
// Module: crate::read::aranges
// Provides: {"ArangeEntryIter"}
// Dependencies: {}
# [doc = " An iterator over the aranges from a `.debug_aranges` section."] # [doc = ""] # [doc = " Can be [used with"] # [doc = " `FallibleIterator`](./index.html#using-with-fallibleiterator)."] # [derive (Debug , Clone)] pub struct ArangeEntryIter < R : Reader > { input : R , encoding : Encoding , }
};
}
