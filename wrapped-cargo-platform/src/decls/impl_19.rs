macro_rules! deps {
    () => {
        CommaSep!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < 'a , T : fmt :: Display > fmt :: Display for CommaSep < 'a , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for (i , v) in self . 0 . iter () . enumerate () { if i > 0 { write ! (f , ", ") ? ; } write ! (f , "{}" , v) ? ; } Ok (()) } }
    };
}

impl_19!();