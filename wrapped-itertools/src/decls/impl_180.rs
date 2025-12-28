macro_rules! deps {
    () => {
        CombinationsGeneric!();
        Combinations!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl < I , Idx > fmt :: Debug for CombinationsGeneric < I , Idx > where I : Iterator + fmt :: Debug , I :: Item : fmt :: Debug , Idx : fmt :: Debug , { debug_fmt_fields ! (Combinations , indices , pool , first) ; }
    };
}

impl_180!();