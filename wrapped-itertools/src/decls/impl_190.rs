macro_rules! deps {
    () => {
        CombinationsWithReplacementGeneric!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        impl < I , Idx > fmt :: Debug for CombinationsWithReplacementGeneric < I , Idx > where I : Iterator + fmt :: Debug , I :: Item : fmt :: Debug + Clone , Idx : fmt :: Debug , { debug_fmt_fields ! (CombinationsWithReplacementGeneric , indices , pool , first) ; }
    };
}

impl_190!();