macro_rules! deps {
    () => {
        UnwrapOrElseFn!();
        Fn1!();
    };
}

macro_rules! impl_1397 {
    () => {
        deps!();
        impl < F , T , E > Fn1 < Result < T , E > > for UnwrapOrElseFn < F > where F : Fn1 < E , Output = T > , { fn call (& self , arg : Result < T , E >) -> Self :: Output { arg . unwrap_or_else (| x | self . 0 . call (x)) } }
    };
}

impl_1397!()