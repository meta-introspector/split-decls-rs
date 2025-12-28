macro_rules! deps {
    () => {
        BlockSizeUser!();
    };
}

macro_rules! Block {
    () => {
        deps!();
        # [doc = " Block on which [`BlockSizeUser`] implementors operate."] pub type Block < B > = Array < u8 , < B as BlockSizeUser > :: BlockSize > ;
    };
}

Block!()