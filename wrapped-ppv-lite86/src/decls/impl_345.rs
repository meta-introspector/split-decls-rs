macro_rules! deps {
    () => {
        GenericMachine!();
    };
}

macro_rules! impl_345 {
    () => {
        deps!();
        impl u32x4 < GenericMachine > for u32x4_generic { }
    };
}

impl_345!();