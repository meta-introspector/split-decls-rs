macro_rules! deps {
    () => {
        LookupEntryIter!();
        Reader!();
        PubStuffParser!();
        PubNamesEntry!();
    };
}

macro_rules! PubNamesEntryIter {
    () => {
        deps!();
        # [doc = " An iterator over the pubnames from a `.debug_pubnames` section."] # [doc = ""] # [doc = " Can be [used with"] # [doc = " `FallibleIterator`](./index.html#using-with-fallibleiterator)."] # [derive (Debug , Clone)] pub struct PubNamesEntryIter < R : Reader > (LookupEntryIter < R , PubStuffParser < R , PubNamesEntry < R > > >) ;
    };
}

PubNamesEntryIter!();