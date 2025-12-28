macro_rules! deps {
    () => {
        FnMut1!();
        InspectErrFn!();
    };
}

macro_rules! impl_1389 {
    () => {
        deps!();
        impl < 'a , F , T , E > FnMut1 < & 'a Result < T , E > > for InspectErrFn < F > where F : FnMut1 < & 'a E , Output = () > , { fn call_mut (& mut self , arg : & 'a Result < T , E >) -> Self :: Output { if let Err (x) = arg { self . 0 . call_mut (x) } } }
    };
}

impl_1389!()