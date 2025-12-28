macro_rules! deps {
    () => {
        FixedBitSet!();
    };
}

macro_rules! BitSetByteSerializer {
    () => {
        deps!();
        struct BitSetByteSerializer < 'a > (& 'a FixedBitSet) ;
    };
}

BitSetByteSerializer!();