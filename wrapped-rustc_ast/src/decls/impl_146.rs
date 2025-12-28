macro_rules! deps {
    () => {
        InlineAsmOptions!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl std :: fmt :: Debug for InlineAsmOptions { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { bitflags :: parser :: to_writer (self , f) } }
    };
}

impl_146!();