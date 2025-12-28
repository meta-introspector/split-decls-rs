macro_rules! deps {
    () => {
        KMergeBy!();
    };
}

macro_rules! impl_333 {
    () => {
        deps!();
        impl < I , F > fmt :: Debug for KMergeBy < I , F > where I : Iterator + fmt :: Debug , I :: Item : fmt :: Debug , { debug_fmt_fields ! (KMergeBy , heap) ; }
    };
}

impl_333!();