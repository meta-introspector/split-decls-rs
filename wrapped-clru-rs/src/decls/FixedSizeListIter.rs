macro_rules! deps {
    () => {
        FixedSizeList!();
    };
}

macro_rules! FixedSizeListIter {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct FixedSizeListIter < 'a , T > { list : & 'a FixedSizeList < T > , front : usize , back : usize , len : usize , }
    };
}

FixedSizeListIter!();