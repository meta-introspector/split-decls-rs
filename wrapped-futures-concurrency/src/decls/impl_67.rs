macro_rules! deps {
    () => {
        WakerArray!();
        ReadinessArrayRef!();
        ReadinessArray!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < const N : usize > WakerArray < N > { # [doc = " Create a new instance of `WakerArray`."] pub (crate) fn new () -> Self { let readiness = ReadinessArray :: new () ; Self { readiness } } pub (crate) fn get (& self , _index : usize) -> Option < & Waker > { self . readiness . parent_waker () } # [doc = " Access the `Readiness`."] pub (crate) fn readiness (& mut self) -> ReadinessArrayRef < '_ , N > { ReadinessArrayRef { inner : & mut self . readiness , } } }
    };
}

impl_67!()