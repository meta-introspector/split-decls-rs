macro_rules! deps {
    () => {
        FnOnce1!();
        InspectFn!();
    };
}

macro_rules! impl_1368 {
    () => {
        deps!();
        impl < F , A > FnOnce1 < A > for InspectFn < F > where F : for < 'a > FnOnce1 < & 'a A , Output = () > , { type Output = A ; fn call_once (self , arg : A) -> Self :: Output { self . 0 . call_once (& arg) ; arg } }
    };
}

impl_1368!()