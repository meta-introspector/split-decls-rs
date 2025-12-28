macro_rules! deps {
    () => {
        SourceKind!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl std :: hash :: Hash for SourceKind { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { core :: mem :: discriminant (self) . hash (state) ; if let SourceKind :: Git (git) = self { git . hash (state) ; } } }
    };
}

impl_27!()