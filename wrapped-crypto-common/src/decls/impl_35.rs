macro_rules! deps {
    () => {
        BlockSizeUser!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < T : BlockSizeUser > BlockSizeUser for & mut T { type BlockSize = T :: BlockSize ; }
    };
}

impl_35!();