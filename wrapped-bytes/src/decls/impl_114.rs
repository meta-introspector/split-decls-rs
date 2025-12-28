macro_rules! deps {
    () => {
        Vtable!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl fmt :: Debug for Vtable { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Vtable") . field ("clone" , & (self . clone as * const ())) . field ("drop" , & (self . drop as * const ())) . finish () } }
    };
}

impl_114!()