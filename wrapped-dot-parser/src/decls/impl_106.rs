macro_rules! deps {
    () => {
        Edge!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        # [cfg (feature = "display")] impl < A > Display for Edge < A > where A : Display , { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , std :: fmt :: Error > { write ! (f , "\"{}\" -> \"{}\" [{}]" , self . from , self . to , self . attr) } }
    };
}

impl_106!()