macro_rules! deps {
    () => {
        Format!();
    };
}

macro_rules! impl_format {
    () => {
        deps!();
        macro_rules ! impl_format { ($ ($ fmt_trait : ident) *) => { $ (impl <'a , I > fmt ::$ fmt_trait for Format <'a , I > where I : Iterator , I :: Item : fmt ::$ fmt_trait , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . format (f , fmt ::$ fmt_trait :: fmt) } }) * } }
    };
}

impl_format!();