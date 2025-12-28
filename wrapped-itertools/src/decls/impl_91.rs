macro_rules! deps {
    () => {
        TakeWhileRef!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl < I , F > fmt :: Debug for TakeWhileRef < '_ , I , F > where I : Iterator + fmt :: Debug , { debug_fmt_fields ! (TakeWhileRef , iter) ; }
    };
}

impl_91!()