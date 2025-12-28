macro_rules! deps {
    () => {
        MapErrFn!();
        FnOnce1!();
    };
}

macro_rules! impl_1378 {
    () => {
        deps!();
        impl < F , T , E > FnOnce1 < Result < T , E > > for MapErrFn < F > where F : FnOnce1 < E > , { type Output = Result < T , F :: Output > ; fn call_once (self , arg : Result < T , E >) -> Self :: Output { arg . map_err (| x | self . 0 . call_once (x)) } }
    };
}

impl_1378!();