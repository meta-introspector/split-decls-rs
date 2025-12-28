macro_rules! deps {
    () => {
        Fill!();
        Rng!();
    };
}

macro_rules! impl_211 {
    () => {
        deps!();
        impl Fill for u8 { fn fill_slice < R : Rng + ? Sized > (this : & mut [Self] , rng : & mut R) { rng . fill_bytes (this) } }
    };
}

impl_211!();