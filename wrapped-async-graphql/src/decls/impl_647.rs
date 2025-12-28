macro_rules! deps {
    () => {
        Timer!();
    };
}

macro_rules! impl_647 {
    () => {
        deps!();
        impl Timer { # [inline] fn new (interval : Duration) -> Self { Self { interval , delay : Delay :: new (interval) , } } # [inline] fn reset (& mut self) { self . delay . reset (self . interval) ; } }
    };
}

impl_647!();