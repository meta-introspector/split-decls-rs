macro_rules! deps {
    () => {
        UnwrapOrElseFn!();
        FnMut1!();
    };
}

macro_rules! impl_1396 {
    () => {
        deps!();
        impl < F , T , E > FnMut1 < Result < T , E > > for UnwrapOrElseFn < F > where F : FnMut1 < E , Output = T > , { fn call_mut (& mut self , arg : Result < T , E >) -> Self :: Output { arg . unwrap_or_else (| x | self . 0 . call_mut (x)) } }
    };
}

impl_1396!();