macro_rules! impl_14 {
    () => {
        impl Display for ReadWrite { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { std :: fmt :: Debug :: fmt (self , f) } }
    };
}

impl_14!()