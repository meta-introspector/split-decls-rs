macro_rules! deps {
    () => {
        Result!();
        Arbitrary!();
        ArbitraryIter!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl < 'a , ElementType : Arbitrary < 'a > > Iterator for ArbitraryIter < 'a , '_ , ElementType > { type Item = Result < ElementType > ; fn next (& mut self) -> Option < Result < ElementType > > { let keep_going = self . u . arbitrary () . unwrap_or (false) ; if keep_going { Some (Arbitrary :: arbitrary (self . u)) } else { None } } }
    };
}

impl_146!()