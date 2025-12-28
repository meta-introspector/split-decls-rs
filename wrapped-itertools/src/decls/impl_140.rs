macro_rules! deps {
    () => {
        Update!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl < I , F > fmt :: Debug for Update < I , F > where I : fmt :: Debug , { debug_fmt_fields ! (Update , iter) ; }
    };
}

impl_140!();