macro_rules! deps {
    () => {
        IMAGE_COR20_HEADER!();
    };
}

macro_rules! impl_231 {
    () => {
        deps!();
        impl Default for IMAGE_COR20_HEADER { fn default () -> Self { unsafe { core :: mem :: zeroed () } } }
    };
}

impl_231!();