macro_rules! deps {
    () => {
        Shared!();
    };
}

macro_rules! impl_212 {
    () => {
        deps!();
        impl Shared { fn is_unique (& self) -> bool { self . ref_count . load (Ordering :: Acquire) == 1 } }
    };
}

impl_212!();