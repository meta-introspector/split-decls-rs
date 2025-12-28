macro_rules! deps {
    () => {
        BitSet!();
    };
}

macro_rules! ByteSet {
    () => {
        deps!();
        # [doc = " A simple set of bytes that is reasonably cheap to copy and allocation free."] # [derive (Clone , Copy , Debug , Default , Eq , PartialEq)] pub (crate) struct ByteSet { bits : BitSet , }
    };
}

ByteSet!()