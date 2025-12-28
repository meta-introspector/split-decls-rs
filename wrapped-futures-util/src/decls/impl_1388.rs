macro_rules! deps {
    () => {
        InspectErrFn!();
        FnOnce1!();
    };
}

macro_rules! impl_1388 {
    () => {
        deps!();
        impl < 'a , F , T , E > FnOnce1 < & 'a Result < T , E > > for InspectErrFn < F > where F : FnOnce1 < & 'a E , Output = () > , { type Output = () ; fn call_once (self , arg : & 'a Result < T , E >) -> Self :: Output { if let Err (x) = arg { self . 0 . call_once (x) } } }
    };
}

impl_1388!()