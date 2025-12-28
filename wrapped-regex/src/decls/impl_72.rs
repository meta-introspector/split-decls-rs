macro_rules! deps {
    () => {
        Replacer!();
        Captures!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < F , T > Replacer for F where F : FnMut (& Captures < '_ >) -> T , T : AsRef < [u8] > , { fn replace_append (& mut self , caps : & Captures < '_ > , dst : & mut Vec < u8 >) { dst . extend_from_slice ((* self) (caps) . as_ref ()) ; } }
    };
}

impl_72!()