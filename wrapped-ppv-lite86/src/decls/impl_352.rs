macro_rules! deps {
    () => {
        GenericMachine!();
    };
}

macro_rules! impl_352 {
    () => {
        deps!();
        impl u32x4x4 < GenericMachine > for u32x4x4_generic { }
    };
}

impl_352!();