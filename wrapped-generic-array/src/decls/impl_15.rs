macro_rules! deps {
    () => {
        ArrayLength!();
        IntoArrayLength!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < const N : usize > IntoArrayLength for Const < N > where Const < N > : ToUInt , typenum :: U < N > : ArrayLength , { type ArrayLength = typenum :: U < N > ; }
    };
}

impl_15!()