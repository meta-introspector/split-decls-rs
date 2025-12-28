macro_rules! deps {
    () => {
        Join!();
    };
}

macro_rules! impl_220 {
    () => {
        deps!();
        impl < Fut , const N : usize > fmt :: Debug for Join < Fut , N > where Fut : Future + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . state . iter ()) . finish () } }
    };
}

impl_220!();