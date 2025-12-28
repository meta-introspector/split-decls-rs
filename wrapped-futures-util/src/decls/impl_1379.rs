macro_rules! deps {
    () => {
        FnMut1!();
        MapErrFn!();
    };
}

macro_rules! impl_1379 {
    () => {
        deps!();
        impl < F , T , E > FnMut1 < Result < T , E > > for MapErrFn < F > where F : FnMut1 < E > , { fn call_mut (& mut self , arg : Result < T , E >) -> Self :: Output { arg . map_err (| x | self . 0 . call_mut (x)) } }
    };
}

impl_1379!()