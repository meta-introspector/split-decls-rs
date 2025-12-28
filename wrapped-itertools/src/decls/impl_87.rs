macro_rules! deps {
    () => {
        Batching!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl < I , F > fmt :: Debug for Batching < I , F > where I : fmt :: Debug , { debug_fmt_fields ! (Batching , iter) ; }
    };
}

impl_87!();