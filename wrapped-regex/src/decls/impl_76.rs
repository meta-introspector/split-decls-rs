macro_rules! deps {
    () => {
        Replacer!();
        NoExpand!();
        Captures!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < 's > Replacer for NoExpand < 's > { fn replace_append (& mut self , _ : & Captures < '_ > , dst : & mut Vec < u8 >) { dst . extend_from_slice (self . 0) ; } fn no_expansion (& mut self) -> Option < Cow < '_ , [u8] > > { Some (Cow :: Borrowed (self . 0)) } }
    };
}

impl_76!();