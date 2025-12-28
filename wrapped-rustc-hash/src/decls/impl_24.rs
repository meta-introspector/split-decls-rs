macro_rules! deps {
    () => {
        FxHasher!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl FxHasher { # [inline] fn add_to_hash (& mut self , i : usize) { self . hash = self . hash . wrapping_add (i) . wrapping_mul (K) ; } }
    };
}

impl_24!();