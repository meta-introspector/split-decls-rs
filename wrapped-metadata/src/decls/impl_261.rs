macro_rules! deps {
    () => {
        IMAGE_SECTION_HEADER!();
    };
}

macro_rules! impl_261 {
    () => {
        deps!();
        impl Default for IMAGE_SECTION_HEADER { fn default () -> Self { unsafe { core :: mem :: zeroed () } } }
    };
}

impl_261!();