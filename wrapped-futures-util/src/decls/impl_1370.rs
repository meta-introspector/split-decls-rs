macro_rules! deps {
    () => {
        InspectFn!();
        Fn1!();
    };
}

macro_rules! impl_1370 {
    () => {
        deps!();
        impl < F , A > Fn1 < A > for InspectFn < F > where F : for < 'a > Fn1 < & 'a A , Output = () > , { fn call (& self , arg : A) -> Self :: Output { self . 0 . call (& arg) ; arg } }
    };
}

impl_1370!()