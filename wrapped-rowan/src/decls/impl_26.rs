macro_rules! deps {
    () => {
        SyntaxNode!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl Hash for SyntaxNode { # [inline] fn hash < H : Hasher > (& self , state : & mut H) { self . data () . key () . hash (state) ; } }
    };
}

impl_26!()