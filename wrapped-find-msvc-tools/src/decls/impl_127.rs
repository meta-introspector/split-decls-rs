macro_rules! deps {
    () => {
        ComPtr!();
        Interface!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl < T > Clone for ComPtr < T > where T : Interface , { fn clone (& self) -> Self { unsafe { self . as_unknown () . AddRef () ; ComPtr :: from_raw (self . 0) } } }
    };
}

impl_127!();