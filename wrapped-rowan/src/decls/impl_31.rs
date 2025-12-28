macro_rules! deps {
    () => {
        SyntaxToken!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl Hash for SyntaxToken { # [inline] fn hash < H : Hasher > (& self , state : & mut H) { self . data () . key () . hash (state) ; } }
    };
}

impl_31!()