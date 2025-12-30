// Generated macro for AttrsIter (struct)
macro_rules! Depcrate_read_unitAttrsIter {
() => {
// Module: crate::read::unit
// Provides: {"AttrsIter"}
// Dependencies: {}
# [doc = " An iterator over a particular entry's attributes."] # [doc = ""] # [doc = " See [the documentation for"] # [doc = " `DebuggingInformationEntry::attrs()`](./struct.DebuggingInformationEntry.html#method.attrs)"] # [doc = " for details."] # [doc = ""] # [doc = " Can be [used with"] # [doc = " `FallibleIterator`](./index.html#using-with-fallibleiterator)."] # [derive (Clone , Copy , Debug)] pub struct AttrsIter < 'abbrev , 'entry , 'unit , R : Reader > { input : R , attributes : & 'abbrev [AttributeSpecification] , entry : & 'entry DebuggingInformationEntry < 'abbrev , 'unit , R > , }
};
}
