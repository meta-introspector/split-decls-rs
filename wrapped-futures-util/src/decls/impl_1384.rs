macro_rules! deps {
    () => {
        FnMut1!();
        InspectOkFn!();
    };
}

macro_rules! impl_1384 {
    () => {
        deps!();
        impl < 'a , F , T , E > FnMut1 < & 'a Result < T , E > > for InspectOkFn < F > where F : FnMut1 < & 'a T , Output = () > , { fn call_mut (& mut self , arg : & 'a Result < T , E >) -> Self :: Output { if let Ok (x) = arg { self . 0 . call_mut (x) } } }
    };
}

impl_1384!()