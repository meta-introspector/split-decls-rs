macro_rules! deps {
    () => {
        Replacer!();
        Captures!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl < 'a > Replacer for Cow < 'a , [u8] > { fn replace_append (& mut self , caps : & Captures < '_ > , dst : & mut Vec < u8 >) { caps . expand (self . as_ref () , dst) ; } fn no_expansion (& mut self) -> Option < Cow < '_ , [u8] > > { no_expansion (self) } }
    };
}

impl_70!()