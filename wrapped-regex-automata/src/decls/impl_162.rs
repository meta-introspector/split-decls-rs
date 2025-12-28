macro_rules! deps {
    () => {
        Accels!();
        AccelTy!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl < A : AsRef < [AccelTy] > > core :: fmt :: Debug for Accels < A > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "Accels(") ? ; let mut list = f . debug_list () ; for a in self . iter () { list . entry (& a) ; } list . finish () ? ; write ! (f , ")") } }
    };
}

impl_162!();