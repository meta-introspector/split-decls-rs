macro_rules! deps {
    () => {
        BufferKindUser!();
    };
}

macro_rules! Buffer {
    () => {
        deps!();
        # [doc = " Buffer type used by type which implements [`BufferKindUser`]."] pub type Buffer < S > = BlockBuffer < < S as BlockSizeUser > :: BlockSize , < S as BufferKindUser > :: BufferKind > ;
    };
}

Buffer!()