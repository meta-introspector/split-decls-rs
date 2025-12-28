macro_rules! deps {
    () => {
        Captures!();
        Replacer!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl Replacer for Vec < u8 > { fn replace_append (& mut self , caps : & Captures < '_ > , dst : & mut Vec < u8 >) { caps . expand (self , dst) ; } fn no_expansion (& mut self) -> Option < Cow < '_ , [u8] > > { no_expansion (self) } }
    };
}

impl_69!();