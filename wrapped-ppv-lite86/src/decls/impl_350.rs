macro_rules! deps {
    () => {
        GenericMachine!();
    };
}

macro_rules! impl_350 {
    () => {
        deps!();
        impl u64x4 < GenericMachine > for u64x4_generic { }
    };
}

impl_350!();