macro_rules! deps {
    () => {
        Group!();
    };
}

macro_rules! impl_218 {
    () => {
        deps!();
        impl Debug for Group { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { Debug :: fmt (& self . inner , formatter) } }
    };
}

impl_218!()