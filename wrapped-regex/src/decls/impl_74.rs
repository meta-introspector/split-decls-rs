macro_rules! deps {
    () => {
        Replacer!();
        ReplacerRef!();
        Captures!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < 'a , R : Replacer + ? Sized + 'a > Replacer for ReplacerRef < 'a , R > { fn replace_append (& mut self , caps : & Captures < '_ > , dst : & mut Vec < u8 >) { self . 0 . replace_append (caps , dst) } fn no_expansion < 'r > (& 'r mut self) -> Option < Cow < 'r , [u8] > > { self . 0 . no_expansion () } }
    };
}

impl_74!();