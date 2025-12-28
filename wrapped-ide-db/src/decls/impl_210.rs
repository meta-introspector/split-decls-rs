macro_rules! deps {
    () => {
        SymbolIndex!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        impl Hash for SymbolIndex { fn hash < H : Hasher > (& self , hasher : & mut H) { self . symbols . hash (hasher) } }
    };
}

impl_210!()