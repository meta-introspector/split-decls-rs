macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl Clone for Data { fn clone (& self) -> Self { Self :: from_slice (self) } }
    };
}

impl_88!()