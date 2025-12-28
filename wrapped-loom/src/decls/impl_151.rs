macro_rules! deps {
    () => {
        StaticValue!();
        Synchronize!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl StaticValue { pub (crate) fn new < T : 'static > (value : T) -> Self { Self { sync : Synchronize :: new () , v : Box :: new (value) , } } pub (crate) fn get < T : 'static > (& self) -> & T { self . v . downcast_ref :: < T > () . expect ("lazy value must downcast to expected type") } }
    };
}

impl_151!()