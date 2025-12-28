macro_rules! deps {
    () => {
        ErrorImpl!();
        Ref!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < E > ErrorImpl < E > { fn erase (& self) -> Ref < ErrorImpl > { Ref :: new (self) . cast :: < ErrorImpl > () } }
    };
}

impl_73!();