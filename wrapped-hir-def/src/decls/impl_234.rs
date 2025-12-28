macro_rules! deps {
    () => {
        AsmOptions!();
    };
}

macro_rules! impl_234 {
    () => {
        deps!();
        impl std :: fmt :: Debug for AsmOptions { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { bitflags :: parser :: to_writer (self , f) } }
    };
}

impl_234!()