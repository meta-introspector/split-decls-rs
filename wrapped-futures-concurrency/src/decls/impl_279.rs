macro_rules! deps {
    () => {
        Race!();
    };
}

macro_rules! impl_279 {
    () => {
        deps!();
        impl < Fut > fmt :: Debug for Race < Fut > where Fut : Future + fmt :: Debug , Fut :: Output : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . futures . iter ()) . finish () } }
    };
}

impl_279!();