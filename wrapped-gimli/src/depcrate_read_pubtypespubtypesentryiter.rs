// Generated macro for PubTypesEntryIter (struct)
macro_rules! Depcrate_read_pubtypesPubTypesEntryIter {
() => {
// Module: crate::read::pubtypes
// Provides: {"PubTypesEntryIter"}
// Dependencies: {}
# [doc = " An iterator over the pubtypes from a `.debug_pubtypes` section."] # [doc = ""] # [doc = " Can be [used with"] # [doc = " `FallibleIterator`](./index.html#using-with-fallibleiterator)."] # [derive (Debug , Clone)] pub struct PubTypesEntryIter < R : Reader > (LookupEntryIter < R , PubStuffParser < R , PubTypesEntry < R > > >) ;
};
}
