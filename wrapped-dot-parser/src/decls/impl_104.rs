macro_rules! deps {
    () => {
        EdgeSet!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        # [cfg (feature = "display")] impl < A > Display for EdgeSet < A > where A : Display , { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , std :: fmt :: Error > { for edge in & self . set { writeln ! (f , "{}" , edge) ? ; } Ok (()) } }
    };
}

impl_104!()