macro_rules! deps {
    () => {
        Powerset!();
    };
}

macro_rules! impl_438 {
    () => {
        deps!();
        impl < I > fmt :: Debug for Powerset < I > where I : Iterator + fmt :: Debug , I :: Item : fmt :: Debug , { debug_fmt_fields ! (Powerset , combs) ; }
    };
}

impl_438!()