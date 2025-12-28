macro_rules! deps {
    () => {
        FnMut1!();
    };
}

macro_rules! impl_1351 {
    () => {
        deps!();
        impl < T , A , R > FnMut1 < A > for T where T : FnMut (A) -> R , { fn call_mut (& mut self , arg : A) -> R { self (arg) } }
    };
}

impl_1351!()