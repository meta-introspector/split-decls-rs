macro_rules! deps {
    () => {
        Unfold!();
    };
}

macro_rules! impl_482 {
    () => {
        deps!();
        impl < St , F > fmt :: Debug for Unfold < St , F > where St : fmt :: Debug , { debug_fmt_fields ! (Unfold , state) ; }
    };
}

impl_482!();