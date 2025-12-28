macro_rules! deps {
    () => {
        GenericMachine!();
    };
}

macro_rules! impl_353 {
    () => {
        deps!();
        impl u64x2x4 < GenericMachine > for u64x2x4_generic { }
    };
}

impl_353!();