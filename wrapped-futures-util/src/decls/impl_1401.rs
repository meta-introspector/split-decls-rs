macro_rules! deps {
    () => {
        FnOnce1!();
        IntoFn!();
    };
}

macro_rules! impl_1401 {
    () => {
        deps!();
        impl < A , T > FnOnce1 < A > for IntoFn < T > where A : Into < T > , { type Output = T ; fn call_once (self , arg : A) -> Self :: Output { arg . into () } }
    };
}

impl_1401!();