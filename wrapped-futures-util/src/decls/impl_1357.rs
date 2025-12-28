macro_rules! deps {
    () => {
        OkFn!();
        FnOnce1!();
    };
}

macro_rules! impl_1357 {
    () => {
        deps!();
        impl < A , E > FnOnce1 < A > for OkFn < E > { type Output = Result < A , E > ; fn call_once (self , arg : A) -> Self :: Output { Ok (arg) } }
    };
}

impl_1357!();