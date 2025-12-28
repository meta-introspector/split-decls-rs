macro_rules! impl_294 {
    () => {
        impl Hash for llvm :: Metadata { fn hash < H : Hasher > (& self , hasher : & mut H) { (self as * const Self) . hash (hasher) ; } }
    };
}

impl_294!()