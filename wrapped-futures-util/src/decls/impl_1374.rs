macro_rules! deps {
    () => {
        FnMut1!();
        MapOkFn!();
    };
}

macro_rules! impl_1374 {
    () => {
        deps!();
        impl < F , T , E > FnMut1 < Result < T , E > > for MapOkFn < F > where F : FnMut1 < T > , { fn call_mut (& mut self , arg : Result < T , E >) -> Self :: Output { arg . map (| x | self . 0 . call_mut (x)) } }
    };
}

impl_1374!()