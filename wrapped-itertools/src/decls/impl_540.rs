macro_rules! deps {
    () => {
        Unique!();
    };
}

macro_rules! impl_540 {
    () => {
        deps!();
        impl < I > fmt :: Debug for Unique < I > where I : Iterator + fmt :: Debug , I :: Item : Hash + Eq + fmt :: Debug + Clone , { debug_fmt_fields ! (Unique , iter) ; }
    };
}

impl_540!();