macro_rules! deps {
    () => {
        ID!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        # [cfg (feature = "display")] impl Display for ID < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , std :: fmt :: Error > { write ! (f , "{}" , self . 0) } }
    };
}

impl_85!()