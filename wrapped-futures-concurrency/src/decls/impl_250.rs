macro_rules! deps {
    () => {
        Join!();
    };
}

macro_rules! impl_250 {
    () => {
        deps!();
        impl < Fut > fmt :: Debug for Join < Fut > where Fut : Future + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . state . iter ()) . finish () } }
    };
}

impl_250!();