macro_rules! deps {
    () => {
        Fn1!();
    };
}

macro_rules! impl_1353 {
    () => {
        deps!();
        impl < T , A , R > Fn1 < A > for T where T : Fn (A) -> R , { fn call (& self , arg : A) -> R { self (arg) } }
    };
}

impl_1353!();