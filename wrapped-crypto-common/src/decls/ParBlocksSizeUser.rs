macro_rules! deps {
    () => {
        BlockSizeUser!();
    };
}

macro_rules! ParBlocksSizeUser {
    () => {
        deps!();
        # [doc = " Types which can process blocks in parallel."] pub trait ParBlocksSizeUser : BlockSizeUser { # [doc = " Number of blocks which can be processed in parallel."] type ParBlocksSize : ArraySize ; }
    };
}

ParBlocksSizeUser!();