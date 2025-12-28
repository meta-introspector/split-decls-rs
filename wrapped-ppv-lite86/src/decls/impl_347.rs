macro_rules! deps {
    () => {
        GenericMachine!();
    };
}

macro_rules! impl_347 {
    () => {
        deps!();
        impl u128x1 < GenericMachine > for u128x1_generic { }
    };
}

impl_347!();