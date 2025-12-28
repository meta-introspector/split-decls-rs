macro_rules! deps {
    () => {
        BlockSizeUser!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < T : BlockSizeUser > BlockSizeUser for & T { type BlockSize = T :: BlockSize ; }
    };
}

impl_10!()