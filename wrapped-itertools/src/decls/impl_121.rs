macro_rules! deps {
    () => {
        FilterOk!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < I , F > fmt :: Debug for FilterOk < I , F > where I : fmt :: Debug , { debug_fmt_fields ! (FilterOk , iter) ; }
    };
}

impl_121!()