macro_rules! deps {
    () => {
        MemberUnraw!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl Hash for MemberUnraw { fn hash < H : Hasher > (& self , hasher : & mut H) { match self { MemberUnraw :: Named (ident) => ident . 0 . unraw () . hash (hasher) , MemberUnraw :: Unnamed (index) => index . hash (hasher) , } } }
    };
}

impl_110!();