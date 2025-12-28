macro_rules! deps {
    () => {
        Fn1!();
        ChainFn!();
    };
}

macro_rules! impl_1362 {
    () => {
        deps!();
        impl < F , G , A > Fn1 < A > for ChainFn < F , G > where F : Fn1 < A > , G : Fn1 < F :: Output > , { fn call (& self , arg : A) -> Self :: Output { self . 1 . call (self . 0 . call (arg)) } }
    };
}

impl_1362!()