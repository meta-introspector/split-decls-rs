macro_rules! deps {
    () => {
        ReprFlags!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl std :: fmt :: Debug for ReprFlags { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { bitflags :: parser :: to_writer (self , f) } }
    };
}

impl_49!()