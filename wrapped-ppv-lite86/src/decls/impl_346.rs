macro_rules! deps {
    () => {
        GenericMachine!();
    };
}

macro_rules! impl_346 {
    () => {
        deps!();
        impl u64x2 < GenericMachine > for u64x2_generic { }
    };
}

impl_346!()