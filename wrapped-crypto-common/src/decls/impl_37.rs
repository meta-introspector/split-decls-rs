macro_rules! deps {
    () => {
        BlockSizes!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < T : ArraySize + sealed :: BlockSizes > BlockSizes for T { }
    };
}

impl_37!();