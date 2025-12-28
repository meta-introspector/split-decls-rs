macro_rules! deps {
    () => {
        Iterate!();
    };
}

macro_rules! impl_486 {
    () => {
        deps!();
        impl < St , F > fmt :: Debug for Iterate < St , F > where St : fmt :: Debug , { debug_fmt_fields ! (Iterate , state) ; }
    };
}

impl_486!();