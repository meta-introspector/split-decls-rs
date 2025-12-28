macro_rules! deps {
    () => {
        Captures!();
        Replacer!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl < 'a > Replacer for & 'a str { fn replace_append (& mut self , caps : & Captures < '_ > , dst : & mut String) { caps . expand (* self , dst) ; } fn no_expansion (& mut self) -> Option < Cow < '_ , str > > { no_expansion (self) } }
    };
}

impl_123!();