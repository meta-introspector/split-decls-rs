macro_rules! deps {
    () => {
        Block!();
        ParBlocksSizeUser!();
    };
}

macro_rules! ParBlocks {
    () => {
        deps!();
        # [doc = " Parallel blocks on which [`ParBlocksSizeUser`] implementors operate."] pub type ParBlocks < T > = Array < Block < T > , < T as ParBlocksSizeUser > :: ParBlocksSize > ;
    };
}

ParBlocks!()