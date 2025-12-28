macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        # [cfg (feature = "display")] impl < A > Display for Node < A > where A : Display , { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , std :: fmt :: Error > { write ! (f , "\"{}\" " , self . id) ? ; if let Some (port) = & self . port { write ! (f , "{}" , port) ? ; } if ! self . attr . is_empty () { write ! (f , "[{}]" , self . attr) ? ; } Ok (()) } }
    };
}

impl_96!()