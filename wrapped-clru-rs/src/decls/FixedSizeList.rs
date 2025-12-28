macro_rules! deps {
    () => {
        FixedSizeListNode!();
    };
}

macro_rules! FixedSizeList {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct FixedSizeList < T > { capacity : usize , nodes : Vec < Option < FixedSizeListNode < T > > > , free : Vec < usize > , front : usize , back : usize , }
    };
}

FixedSizeList!();