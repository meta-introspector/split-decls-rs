macro_rules! deps {
    () => {
        MapErrFn!();
        Fn1!();
    };
}

macro_rules! impl_1380 {
    () => {
        deps!();
        impl < F , T , E > Fn1 < Result < T , E > > for MapErrFn < F > where F : Fn1 < E > , { fn call (& self , arg : Result < T , E >) -> Self :: Output { arg . map_err (| x | self . 0 . call (x)) } }
    };
}

impl_1380!()