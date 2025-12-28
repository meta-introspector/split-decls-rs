macro_rules! deps {
    () => {
        Hasher!();
        FnvHasher!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl crate :: Hasher for FnvHasher { # [inline] fn finish32 (& self) -> u32 { self . state } }
    };
}

impl_4!();