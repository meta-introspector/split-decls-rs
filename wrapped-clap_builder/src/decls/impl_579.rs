macro_rules! deps {
    () => {
        Colorizer!();
        Result!();
    };
}

macro_rules! impl_579 {
    () => {
        deps!();
        # [doc = " Color-unaware printing. Never uses coloring."] impl std :: fmt :: Display for Colorizer { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . content . fmt (f) } }
    };
}

impl_579!()