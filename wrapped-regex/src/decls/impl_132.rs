macro_rules! deps {
    () => {
        Replacer!();
        Captures!();
        NoExpand!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl < 's > Replacer for NoExpand < 's > { fn replace_append (& mut self , _ : & Captures < '_ > , dst : & mut String) { dst . push_str (self . 0) ; } fn no_expansion (& mut self) -> Option < Cow < '_ , str > > { Some (Cow :: Borrowed (self . 0)) } }
    };
}

impl_132!()