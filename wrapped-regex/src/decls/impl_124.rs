macro_rules! deps {
    () => {
        Captures!();
        Replacer!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl < 'a > Replacer for & 'a String { fn replace_append (& mut self , caps : & Captures < '_ > , dst : & mut String) { self . as_str () . replace_append (caps , dst) } fn no_expansion (& mut self) -> Option < Cow < '_ , str > > { no_expansion (self) } }
    };
}

impl_124!();