macro_rules! deps {
    () => {
        RaceOk!();
    };
}

macro_rules! impl_335 {
    () => {
        deps!();
        impl < Fut , T , E > fmt :: Debug for RaceOk < Fut , T , E > where Fut : Future < Output = Result < T , E > > + fmt :: Debug , Fut :: Output : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . elems . iter ()) . finish () } }
    };
}

impl_335!();