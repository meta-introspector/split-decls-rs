macro_rules! deps {
    () => {
        ComPtr!();
        Interface!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl < T > Drop for ComPtr < T > where T : Interface , { fn drop (& mut self) { unsafe { self . as_unknown () . Release () ; } } }
    };
}

impl_128!();