macro_rules! deps {
    () => {
        Reader!();
        Encoding!();
    };
}

macro_rules! AddrEntryIter {
    () => {
        deps!();
        # [doc = " An iterator over the addresses from a `.debug_addr` section."] # [doc = ""] # [doc = " Can be [used with"] # [doc = " `FallibleIterator`](./index.html#using-with-fallibleiterator)."] # [derive (Debug , Clone)] pub struct AddrEntryIter < R : Reader > { input : R , encoding : Encoding , }
    };
}

AddrEntryIter!();