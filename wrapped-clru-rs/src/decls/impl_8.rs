macro_rules! deps {
    () => {
        FixedSizeListIter!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < T > Clone for FixedSizeListIter < '_ , T > { fn clone (& self) -> Self { Self { list : self . list , front : self . front , back : self . back , len : self . len , } } }
    };
}

impl_8!()