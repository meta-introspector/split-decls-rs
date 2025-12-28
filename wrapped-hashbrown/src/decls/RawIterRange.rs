macro_rules! deps {
    () => {
        RawIter!();
        BitMaskIter!();
        Bucket!();
    };
}

macro_rules! RawIterRange {
    () => {
        deps!();
        # [doc = " Iterator over a sub-range of a table. Unlike `RawIter` this iterator does"] # [doc = " not track an item count."] pub (crate) struct RawIterRange < T > { current_group : BitMaskIter , data : Bucket < T > , next_ctrl : * const u8 , end : * const u8 , }
    };
}

RawIterRange!()