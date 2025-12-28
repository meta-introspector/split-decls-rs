macro_rules! deps {
    () => {
        Chunks!();
    };
}

macro_rules! impl_280 {
    () => {
        deps!();
        impl < 'a , I > Debug for Chunks < 'a , I > where I : Iterator + Debug , I :: Item : Debug , { debug_fmt_fields ! (Chunks , parent) ; }
    };
}

impl_280!();