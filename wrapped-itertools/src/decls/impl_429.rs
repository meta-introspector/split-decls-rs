macro_rules! deps {
    () => {
        Permutations!();
    };
}

macro_rules! impl_429 {
    () => {
        deps!();
        impl < I > fmt :: Debug for Permutations < I > where I : Iterator + fmt :: Debug , I :: Item : fmt :: Debug , { debug_fmt_fields ! (Permutations , vals , state) ; }
    };
}

impl_429!()