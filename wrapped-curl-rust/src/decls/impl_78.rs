macro_rules! deps {
    () => {
        Easy2!();
        Easy!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < H : fmt :: Debug > fmt :: Debug for Easy2 < H > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Easy") . field ("handle" , & self . inner . handle) . field ("handler" , & self . inner . handler) . finish () } }
    };
}

impl_78!();