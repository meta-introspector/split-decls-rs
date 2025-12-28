macro_rules! deps {
    () => {
        IMAGE_OPTIONAL_HEADER64!();
    };
}

macro_rules! impl_257 {
    () => {
        deps!();
        impl Default for IMAGE_OPTIONAL_HEADER64 { fn default () -> Self { unsafe { core :: mem :: zeroed () } } }
    };
}

impl_257!();