macro_rules! deps {
    () => {
        Utf8BoundedMap!();
        Utf8State!();
    };
}

macro_rules! impl_477 {
    () => {
        deps!();
        impl Utf8State { fn new () -> Utf8State { Utf8State { compiled : Utf8BoundedMap :: new (10_000) , uncompiled : vec ! [] } } fn clear (& mut self) { self . compiled . clear () ; self . uncompiled . clear () ; } }
    };
}

impl_477!();