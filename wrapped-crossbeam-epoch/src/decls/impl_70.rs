macro_rules! deps {
    () => {
        Local!();
        LocalHandle!();
        Collector!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl Collector { # [doc = " Creates a new collector."] pub fn new () -> Self { Self :: default () } # [doc = " Registers a new handle for the collector."] pub fn register (& self) -> LocalHandle { Local :: register (self) } }
    };
}

impl_70!()