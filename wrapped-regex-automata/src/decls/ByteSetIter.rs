macro_rules! deps {
    () => {
        ByteSet!();
    };
}

macro_rules! ByteSetIter {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct ByteSetIter < 'a > { set : & 'a ByteSet , b : usize , }
    };
}

ByteSetIter!();