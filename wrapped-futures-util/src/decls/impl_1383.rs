macro_rules! deps {
    () => {
        FnOnce1!();
        InspectOkFn!();
    };
}

macro_rules! impl_1383 {
    () => {
        deps!();
        impl < 'a , F , T , E > FnOnce1 < & 'a Result < T , E > > for InspectOkFn < F > where F : FnOnce1 < & 'a T , Output = () > , { type Output = () ; fn call_once (self , arg : & 'a Result < T , E >) -> Self :: Output { if let Ok (x) = arg { self . 0 . call_once (x) } } }
    };
}

impl_1383!();