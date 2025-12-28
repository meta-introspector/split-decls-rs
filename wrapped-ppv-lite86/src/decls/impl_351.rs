macro_rules! deps {
    () => {
        GenericMachine!();
    };
}

macro_rules! impl_351 {
    () => {
        deps!();
        impl u128x2 < GenericMachine > for u128x2_generic { }
    };
}

impl_351!()