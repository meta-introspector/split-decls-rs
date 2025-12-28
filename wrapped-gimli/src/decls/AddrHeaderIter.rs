macro_rules! deps {
    () => {
        Reader!();
        DebugAddrOffset!();
    };
}

macro_rules! AddrHeaderIter {
    () => {
        deps!();
        # [doc = " An iterator over the headers of a `.debug_addr` section."] # [derive (Clone , Debug)] pub struct AddrHeaderIter < R : Reader > { input : R , offset : DebugAddrOffset < R :: Offset > , }
    };
}

AddrHeaderIter!();