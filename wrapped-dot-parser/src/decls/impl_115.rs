macro_rules! deps {
    () => {
        IDEq!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        # [cfg (feature = "display")] impl Display for IDEq { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , std :: fmt :: Error > { write ! (f , "{} = {}" , self . lhs , self . rhs) } }
    };
}

impl_115!();