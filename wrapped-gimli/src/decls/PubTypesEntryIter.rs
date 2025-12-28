macro_rules! deps {
    () => {
        LookupEntryIter!();
        PubStuffParser!();
        PubTypesEntry!();
        Reader!();
    };
}

macro_rules! PubTypesEntryIter {
    () => {
        deps!();
        # [doc = " An iterator over the pubtypes from a `.debug_pubtypes` section."] # [doc = ""] # [doc = " Can be [used with"] # [doc = " `FallibleIterator`](./index.html#using-with-fallibleiterator)."] # [derive (Debug , Clone)] pub struct PubTypesEntryIter < R : Reader > (LookupEntryIter < R , PubStuffParser < R , PubTypesEntry < R > > >) ;
    };
}

PubTypesEntryIter!()