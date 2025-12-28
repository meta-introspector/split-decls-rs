macro_rules! deps {
    () => {
        UniformBlocks!();
    };
}

macro_rules! impl_299 {
    () => {
        deps!();
        impl < I > UniformBlocks < I > { pub (super) fn new (base : I , block_size : usize) -> Self { Self { base , block_size } } }
    };
}

impl_299!()