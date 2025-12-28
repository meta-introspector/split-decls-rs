macro_rules! impl_614 {
    () => {
        impl Hash for Value { fn hash < H : Hasher > (& self , hasher : & mut H) { (self as * const Self) . hash (hasher) ; } }
    };
}

impl_614!();