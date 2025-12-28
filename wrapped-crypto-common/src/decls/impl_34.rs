macro_rules! deps {
    () => {
        BlockSizeUser!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < T : BlockSizeUser > BlockSizeUser for & T { type BlockSize = T :: BlockSize ; }
    };
}

impl_34!()