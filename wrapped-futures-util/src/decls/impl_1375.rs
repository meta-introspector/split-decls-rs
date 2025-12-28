macro_rules! deps {
    () => {
        MapOkFn!();
        Fn1!();
    };
}

macro_rules! impl_1375 {
    () => {
        deps!();
        impl < F , T , E > Fn1 < Result < T , E > > for MapOkFn < F > where F : Fn1 < T > , { fn call (& self , arg : Result < T , E >) -> Self :: Output { arg . map (| x | self . 0 . call (x)) } }
    };
}

impl_1375!();