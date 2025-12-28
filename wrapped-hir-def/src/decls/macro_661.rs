macro_rules! deps {
    () => {
        BlockLoc!();
    };
}

macro_rules! macro_661 {
    () => {
        deps!();
        impl_intern ! (BlockId , BlockLoc , intern_block , lookup_intern_block) ;
    };
}

macro_661!();