macro_rules! deps {
    () => {
        FnMut1!();
        ChainFn!();
    };
}

macro_rules! impl_1361 {
    () => {
        deps!();
        impl < F , G , A > FnMut1 < A > for ChainFn < F , G > where F : FnMut1 < A > , G : FnMut1 < F :: Output > , { fn call_mut (& mut self , arg : A) -> Self :: Output { self . 1 . call_mut (self . 0 . call_mut (arg)) } }
    };
}

impl_1361!()