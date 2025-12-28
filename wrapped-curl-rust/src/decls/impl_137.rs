macro_rules! deps {
    () => {
        EasyHandle!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl fmt :: Debug for EasyHandle { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . easy . fmt (f) } }
    };
}

impl_137!();