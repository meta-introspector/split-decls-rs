macro_rules! deps {
    () => {
        ExponentialBlocks!();
    };
}

macro_rules! impl_295 {
    () => {
        deps!();
        impl < I > ExponentialBlocks < I > { pub (super) fn new (base : I) -> Self { Self { base } } }
    };
}

impl_295!()