macro_rules! deps {
    () => {
        FnOnce1!();
    };
}

macro_rules! impl_1349 {
    () => {
        deps!();
        impl < T , A , R > FnOnce1 < A > for T where T : FnOnce (A) -> R , { type Output = R ; fn call_once (self , arg : A) -> R { self (arg) } }
    };
}

impl_1349!();