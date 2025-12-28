macro_rules! deps {
    () => {
        OrderingOrBool!();
        MergeBy!();
    };
}

macro_rules! impl_362 {
    () => {
        deps!();
        impl < I , J , F > FusedIterator for MergeBy < I , J , F > where I : Iterator , J : Iterator , F : OrderingOrBool < I :: Item , J :: Item > , { }
    };
}

impl_362!();