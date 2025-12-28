macro_rules! deps {
    () => {
        MergeBy!();
    };
}

macro_rules! impl_360 {
    () => {
        deps!();
        impl < I , J , F > fmt :: Debug for MergeBy < I , J , F > where I : Iterator + fmt :: Debug , I :: Item : fmt :: Debug , J : Iterator + fmt :: Debug , J :: Item : fmt :: Debug , { debug_fmt_fields ! (MergeBy , left , right) ; }
    };
}

impl_360!();