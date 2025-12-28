macro_rules! deps {
    () => {
        GenericMachine!();
    };
}

macro_rules! impl_348 {
    () => {
        deps!();
        impl u32x4x2 < GenericMachine > for u32x4x2_generic { }
    };
}

impl_348!();