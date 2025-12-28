macro_rules! deps {
    () => {
        BlockSizeUser!();
    };
}

macro_rules! AddBlockSize {
    () => {
        deps!();
        # [doc = " Alias for `AddBlockSize<A, B> = Sum<T, B::BlockSize>`"] pub type AddBlockSize < T , B > = Sum < T , < B as BlockSizeUser > :: BlockSize > ;
    };
}

AddBlockSize!();