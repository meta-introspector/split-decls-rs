macro_rules! deps {
    () => {
        Pairs!();
        RuleType!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl < R : RuleType > fmt :: Display for Pairs < '_ , R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "[{}]" , self . clone () . map (| pair | format ! ("{}" , pair)) . collect ::< Vec < _ >> () . join (", ")) } }
    };
}

impl_55!()