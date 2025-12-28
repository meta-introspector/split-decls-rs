macro_rules! deps {
    () => {
        InspectOkFn!();
        Fn1!();
    };
}

macro_rules! impl_1385 {
    () => {
        deps!();
        impl < 'a , F , T , E > Fn1 < & 'a Result < T , E > > for InspectOkFn < F > where F : Fn1 < & 'a T , Output = () > , { fn call (& self , arg : & 'a Result < T , E >) -> Self :: Output { if let Ok (x) = arg { self . 0 . call (x) } } }
    };
}

impl_1385!()