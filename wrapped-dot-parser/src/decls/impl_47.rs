macro_rules! deps {
    () => {
        AList!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        # [cfg (feature = "display")] impl < A > Display for AList < A > where A : Display , { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , std :: fmt :: Error > { for attr in & self . elems { write ! (f , "{}; " , attr) ? ; } Ok (()) } }
    };
}

impl_47!();