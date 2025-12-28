macro_rules! deps {
    () => {
        Printer!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl Printer { pub fn lifetime (& mut self , lifetime : & Lifetime) { self . word ("'") ; self . ident (& lifetime . ident) ; } }
    };
}

impl_57!()