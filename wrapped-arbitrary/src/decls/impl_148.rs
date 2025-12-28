macro_rules! deps {
    () => {
        Result!();
        ArbitraryTakeRestIter!();
        Arbitrary!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl < 'a , ElementType : Arbitrary < 'a > > Iterator for ArbitraryTakeRestIter < 'a , ElementType > { type Item = Result < ElementType > ; fn next (& mut self) -> Option < Result < ElementType > > { let keep_going = self . u . arbitrary () . unwrap_or (false) ; if keep_going { Some (Arbitrary :: arbitrary (& mut self . u)) } else { None } } }
    };
}

impl_148!()