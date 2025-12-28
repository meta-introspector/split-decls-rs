macro_rules! deps {
    () => {
        IntoIter!();
        BitMask!();
        BitMaskIter!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl IntoIterator for BitMask { type Item = usize ; type IntoIter = BitMaskIter ; # [inline] fn into_iter (self) -> BitMaskIter { BitMaskIter (BitMask (self . 0 & BITMASK_ITER_MASK)) } }
    };
}

impl_9!()