macro_rules! deps {
    () => {
        FilterMapOk!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl < I , F > fmt :: Debug for FilterMapOk < I , F > where I : fmt :: Debug , { debug_fmt_fields ! (FilterMapOk , iter) ; }
    };
}

impl_127!();