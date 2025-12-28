macro_rules! deps {
    () => {
        FnOnce1!();
        UnwrapOrElseFn!();
    };
}

macro_rules! impl_1395 {
    () => {
        deps!();
        impl < F , T , E > FnOnce1 < Result < T , E > > for UnwrapOrElseFn < F > where F : FnOnce1 < E , Output = T > , { type Output = T ; fn call_once (self , arg : Result < T , E >) -> Self :: Output { arg . unwrap_or_else (| x | self . 0 . call_once (x)) } }
    };
}

impl_1395!()