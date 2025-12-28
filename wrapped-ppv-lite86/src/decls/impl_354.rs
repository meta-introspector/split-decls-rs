macro_rules! deps {
    () => {
        GenericMachine!();
    };
}

macro_rules! impl_354 {
    () => {
        deps!();
        impl u128x4 < GenericMachine > for u128x4_generic { }
    };
}

impl_354!()