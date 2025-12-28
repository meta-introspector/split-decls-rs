macro_rules! deps {
    () => {
        IMAGE_OPTIONAL_HEADER32!();
    };
}

macro_rules! impl_255 {
    () => {
        deps!();
        impl Default for IMAGE_OPTIONAL_HEADER32 { fn default () -> Self { unsafe { core :: mem :: zeroed () } } }
    };
}

impl_255!();