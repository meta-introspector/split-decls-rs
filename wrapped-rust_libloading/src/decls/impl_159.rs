macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl < T > fmt :: Debug for Symbol < '_ , T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . inner . fmt (f) } }
    };
}

impl_159!();