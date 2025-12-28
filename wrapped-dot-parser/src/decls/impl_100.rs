macro_rules! deps {
    () => {
        NodeSet!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        # [cfg (feature = "display")] impl < A > Display for NodeSet < A > where A : Display , { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , std :: fmt :: Error > { for node in self . set . values () { writeln ! (f , "{}" , node) ? ; } Ok (()) } }
    };
}

impl_100!()