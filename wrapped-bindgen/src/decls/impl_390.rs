macro_rules! deps {
    () => {
        IMAGE_DOS_HEADER!();
    };
}

macro_rules! impl_390 {
    () => {
        deps!();
        impl Default for IMAGE_DOS_HEADER { fn default () -> Self { unsafe { core :: mem :: zeroed () } } }
    };
}

impl_390!()