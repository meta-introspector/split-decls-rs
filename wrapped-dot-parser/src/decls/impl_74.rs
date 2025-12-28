macro_rules! deps {
    () => {
        Port!();
        ID!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        # [cfg (feature = "display")] impl Display for Port { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , std :: fmt :: Error > { match self { Port :: ID (name , Some (cpss)) => { write ! (f , ": {name} : {cpss}") } Port :: ID (name , None) => { write ! (f , ": {name}") } Port :: Compass (cpss) => { write ! (f , ": {}" , cpss) } } } }
    };
}

impl_74!();