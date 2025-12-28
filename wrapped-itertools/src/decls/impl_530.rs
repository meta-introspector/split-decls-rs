macro_rules! deps {
    () => {
        UniqueBy!();
    };
}

macro_rules! impl_530 {
    () => {
        deps!();
        impl < I , V , F > fmt :: Debug for UniqueBy < I , V , F > where I : Iterator + fmt :: Debug , V : fmt :: Debug + Hash + Eq , { debug_fmt_fields ! (UniqueBy , iter , used) ; }
    };
}

impl_530!();