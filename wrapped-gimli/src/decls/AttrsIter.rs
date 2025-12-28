macro_rules! deps {
    () => {
        Reader!();
        DebuggingInformationEntry!();
        AttributeSpecification!();
    };
}

macro_rules! AttrsIter {
    () => {
        deps!();
        # [doc = " An iterator over a particular entry's attributes."] # [doc = ""] # [doc = " See [the documentation for"] # [doc = " `DebuggingInformationEntry::attrs()`](./struct.DebuggingInformationEntry.html#method.attrs)"] # [doc = " for details."] # [doc = ""] # [doc = " Can be [used with"] # [doc = " `FallibleIterator`](./index.html#using-with-fallibleiterator)."] # [derive (Clone , Copy , Debug)] pub struct AttrsIter < 'abbrev , 'entry , 'unit , R : Reader > { input : R , attributes : & 'abbrev [AttributeSpecification] , entry : & 'entry DebuggingInformationEntry < 'abbrev , 'unit , R > , }
    };
}

AttrsIter!()