macro_rules! deps {
    () => {
        Positions!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl < I , F > fmt :: Debug for Positions < I , F > where I : fmt :: Debug , { debug_fmt_fields ! (Positions , iter) ; }
    };
}

impl_134!();