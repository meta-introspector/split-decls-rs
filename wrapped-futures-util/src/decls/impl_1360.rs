macro_rules! deps {
    () => {
        FnOnce1!();
        ChainFn!();
    };
}

macro_rules! impl_1360 {
    () => {
        deps!();
        impl < F , G , A > FnOnce1 < A > for ChainFn < F , G > where F : FnOnce1 < A > , G : FnOnce1 < F :: Output > , { type Output = G :: Output ; fn call_once (self , arg : A) -> Self :: Output { self . 1 . call_once (self . 0 . call_once (arg)) } }
    };
}

impl_1360!();