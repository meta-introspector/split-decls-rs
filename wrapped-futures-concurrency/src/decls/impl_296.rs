macro_rules! deps {
    () => {
        RaceOk!();
    };
}

macro_rules! impl_296 {
    () => {
        deps!();
        impl < Fut , T , E , const N : usize > fmt :: Debug for RaceOk < Fut , T , E , N > where Fut : Future < Output = Result < T , E > > + fmt :: Debug , Fut :: Output : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . futures . iter ()) . finish () } }
    };
}

impl_296!()