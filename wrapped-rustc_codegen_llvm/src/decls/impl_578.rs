macro_rules! impl_578 {
    () => {
        impl Hash for Type { fn hash < H : Hasher > (& self , state : & mut H) { ptr :: hash (self , state) ; } }
    };
}

impl_578!()