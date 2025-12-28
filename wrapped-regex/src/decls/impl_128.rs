macro_rules! deps {
    () => {
        Captures!();
        Replacer!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl < F , T > Replacer for F where F : FnMut (& Captures < '_ >) -> T , T : AsRef < str > , { fn replace_append (& mut self , caps : & Captures < '_ > , dst : & mut String) { dst . push_str ((* self) (caps) . as_ref ()) ; } }
    };
}

impl_128!()