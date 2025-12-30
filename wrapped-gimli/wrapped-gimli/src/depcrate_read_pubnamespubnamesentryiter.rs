// Generated macro for PubNamesEntryIter (struct)
macro_rules! Depcrate_read_pubnamesPubNamesEntryIter {
() => {
// Module: crate::read::pubnames
// Provides: {"PubNamesEntryIter"}
// Dependencies: {}
# [doc = " An iterator over the pubnames from a `.debug_pubnames` section."] # [doc = ""] # [doc = " Can be [used with"] # [doc = " `FallibleIterator`](./index.html#using-with-fallibleiterator)."] # [derive (Debug , Clone)] pub struct PubNamesEntryIter < R : Reader > (LookupEntryIter < R , PubStuffParser < R , PubNamesEntry < R > > >) ;
};
}
