macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! impl_502 {
    () => {
        deps!();
        impl Hash for Encoding { # [inline] fn hash < H : Hasher > (& self , state : & mut H) { (self as * const Encoding) . hash (state) ; } }
    };
}

impl_502!();