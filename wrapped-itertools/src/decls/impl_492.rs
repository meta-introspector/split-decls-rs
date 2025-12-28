macro_rules! deps {
    () => {
        TakeWhileInclusive!();
    };
}

macro_rules! impl_492 {
    () => {
        deps!();
        impl < I , F > fmt :: Debug for TakeWhileInclusive < I , F > where I : Iterator + fmt :: Debug , { debug_fmt_fields ! (TakeWhileInclusive , iter , done) ; }
    };
}

impl_492!();