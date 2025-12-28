macro_rules! deps {
    () => {
        Group!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl Debug for Group { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { Debug :: fmt (& self . inner , formatter) } }
    };
}

impl_48!()