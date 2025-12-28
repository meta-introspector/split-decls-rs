macro_rules! deps {
    () => {
        Select!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        impl fmt :: Debug for Select < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . pad ("Select { .. }") } }
    };
}

impl_182!()