macro_rules! deps {
    () => {
        BitMask!();
    };
}

macro_rules! BitMaskIter {
    () => {
        deps!();
        # [doc = " Iterator over the contents of a `BitMask`, returning the indices of set"] # [doc = " bits."] # [derive (Clone)] pub (crate) struct BitMaskIter (pub (crate) BitMask) ;
    };
}

BitMaskIter!();