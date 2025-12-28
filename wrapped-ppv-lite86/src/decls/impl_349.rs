macro_rules! deps {
    () => {
        GenericMachine!();
    };
}

macro_rules! impl_349 {
    () => {
        deps!();
        impl u64x2x2 < GenericMachine > for u64x2x2_generic { }
    };
}

impl_349!();