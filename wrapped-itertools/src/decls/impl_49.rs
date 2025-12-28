macro_rules! deps {
    () => {
        MapSpecialCaseFnOk!();
        MapSpecialCaseFn!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < F , T , U , E > MapSpecialCaseFn < Result < T , E > > for MapSpecialCaseFnOk < F > where F : FnMut (T) -> U , { type Out = Result < U , E > ; fn call (& mut self , t : Result < T , E >) -> Self :: Out { t . map (| v | self . 0 (v)) } }
    };
}

impl_49!();