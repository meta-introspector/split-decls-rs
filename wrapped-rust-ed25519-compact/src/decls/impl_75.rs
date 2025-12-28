macro_rules! deps {
    () => {
        Mem!();
        VerifyingState!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl Drop for VerifyingState { fn drop (& mut self) { Mem :: wipe (self . signature . 0) ; } }
    };
}

impl_75!();