macro_rules! deps {
    () => {
        Map!();
        Result!();
        Row!();
        Error!();
    };
}

macro_rules! impl_216 {
    () => {
        deps!();
        impl < F , B > FallibleIterator for Map < '_ , F > where F : FnMut (& Row < '_ >) -> Result < B > , { type Error = Error ; type Item = B ; # [inline] fn next (& mut self) -> Result < Option < B > > { match self . rows . next () ? { Some (v) => Ok (Some ((self . f) (v) ?)) , None => Ok (None) , } } }
    };
}

impl_216!()