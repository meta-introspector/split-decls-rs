macro_rules! deps {
    () => {
        Replacer!();
        Captures!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl < 'a > Replacer for & 'a Cow < 'a , str > { fn replace_append (& mut self , caps : & Captures < '_ > , dst : & mut String) { self . as_ref () . replace_append (caps , dst) } fn no_expansion (& mut self) -> Option < Cow < '_ , str > > { no_expansion (self) } }
    };
}

impl_127!()