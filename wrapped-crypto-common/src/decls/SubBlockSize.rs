macro_rules! deps {
    () => {
        BlockSizeUser!();
    };
}

macro_rules! SubBlockSize {
    () => {
        deps!();
        # [doc = " Alias for `SubBlockSize<A, B> = Diff<T, B::BlockSize>`"] pub type SubBlockSize < T , B > = Diff < T , < B as BlockSizeUser > :: BlockSize > ;
    };
}

SubBlockSize!();