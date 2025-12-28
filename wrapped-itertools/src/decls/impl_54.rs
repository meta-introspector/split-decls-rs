macro_rules! deps {
    () => {
        MapSpecialCaseFnInto!();
        MapSpecialCaseFn!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < T : Into < U > , U > MapSpecialCaseFn < T > for MapSpecialCaseFnInto < U > { type Out = U ; fn call (& mut self , t : T) -> Self :: Out { t . into () } }
    };
}

impl_54!()