macro_rules! deps {
    () => {
        InspectErrFn!();
        Fn1!();
    };
}

macro_rules! impl_1390 {
    () => {
        deps!();
        impl < 'a , F , T , E > Fn1 < & 'a Result < T , E > > for InspectErrFn < F > where F : Fn1 < & 'a E , Output = () > , { fn call (& self , arg : & 'a Result < T , E >) -> Self :: Output { if let Err (x) = arg { self . 0 . call (x) } } }
    };
}

impl_1390!()