macro_rules! deps {
    () => {
        InspectFn!();
        FnMut1!();
    };
}

macro_rules! impl_1369 {
    () => {
        deps!();
        impl < F , A > FnMut1 < A > for InspectFn < F > where F : for < 'a > FnMut1 < & 'a A , Output = () > , { fn call_mut (& mut self , arg : A) -> Self :: Output { self . 0 . call_mut (& arg) ; arg } }
    };
}

impl_1369!();