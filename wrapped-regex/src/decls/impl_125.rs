macro_rules! deps {
    () => {
        Captures!();
        Replacer!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl Replacer for String { fn replace_append (& mut self , caps : & Captures < '_ > , dst : & mut String) { self . as_str () . replace_append (caps , dst) } fn no_expansion (& mut self) -> Option < Cow < '_ , str > > { no_expansion (self) } }
    };
}

impl_125!();