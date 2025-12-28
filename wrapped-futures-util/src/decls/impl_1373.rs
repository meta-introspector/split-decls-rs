macro_rules! deps {
    () => {
        MapOkFn!();
        FnOnce1!();
    };
}

macro_rules! impl_1373 {
    () => {
        deps!();
        impl < F , T , E > FnOnce1 < Result < T , E > > for MapOkFn < F > where F : FnOnce1 < T > , { type Output = Result < F :: Output , E > ; fn call_once (self , arg : Result < T , E >) -> Self :: Output { arg . map (| x | self . 0 . call_once (x)) } }
    };
}

impl_1373!()