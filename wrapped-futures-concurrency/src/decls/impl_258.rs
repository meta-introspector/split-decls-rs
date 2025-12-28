macro_rules! deps {
    () => {
        Race!();
    };
}

macro_rules! impl_258 {
    () => {
        deps!();
        impl < Fut , const N : usize > fmt :: Debug for Race < Fut , N > where Fut : Future + fmt :: Debug , Fut :: Output : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . futures . iter ()) . finish () } }
    };
}

impl_258!();