macro_rules! deps {
    () => {
        ComPtr!();
        Interface!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < T > Deref for ComPtr < T > where T : Interface , { type Target = T ; fn deref (& self) -> & T { unsafe { & * self . 0 } } }
    };
}

impl_126!();