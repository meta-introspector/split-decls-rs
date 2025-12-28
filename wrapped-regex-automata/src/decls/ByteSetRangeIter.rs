macro_rules! deps {
    () => {
        ByteSet!();
    };
}

macro_rules! ByteSetRangeIter {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct ByteSetRangeIter < 'a > { set : & 'a ByteSet , b : usize , }
    };
}

ByteSetRangeIter!();