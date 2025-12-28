macro_rules! deps {
    () => {
        ReplacerRef!();
        Captures!();
        Replacer!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl < 'a , R : Replacer + ? Sized + 'a > Replacer for ReplacerRef < 'a , R > { fn replace_append (& mut self , caps : & Captures < '_ > , dst : & mut String) { self . 0 . replace_append (caps , dst) } fn no_expansion (& mut self) -> Option < Cow < '_ , str > > { self . 0 . no_expansion () } }
    };
}

impl_130!()